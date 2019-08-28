/*
 * File: src/monitor/mod.rs
 * Date: 20.08.2019
 * Author: MarkAtk
 *
 * MIT License
 *
 * Copyright (c) 2019 MarkAtk
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
 * of the Software, and to permit persons to whom the Software is furnished to do
 * so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::io;
use std::thread;
use std::sync::mpsc;
use clap::{ArgMatches, App, SubCommand};
use crate::commands;
use serial_unit_testing::utils;
use serial_unit_testing::serial::Serial;

mod event;
mod ui;

use event::Event;
use std::time::Duration;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let (io_tx, io_rx) = mpsc::channel();

    let mut monitor = match ui::Monitor::new(io_tx) {
        Ok(monitor) => monitor,
        Err(e) => return Err(e.to_string())
    };

    // open serial port
    let (settings, port_name) = commands::get_serial_settings(matches).unwrap();

    let mut serial = match Serial::open_with_settings(port_name, &settings) {
        Ok(serial) => serial,
        Err(e) => return Err(format!("Error opening port {:?}", e))
    };

    // start thread for receiving from and sending to serial port
    {
        let text_format = commands::get_text_output_format(matches);
        let tx = monitor.ui_tx.clone();

        thread::spawn(move || {
            loop {
                match serial.read_with_timeout(Duration::from_millis(10)) {
                    Ok(bytes) => {
                        let result = utils::radix_string(bytes, &text_format);

                        if let Err(_) = tx.send(Event::Output(result)) {
                            return;
                        }
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    // TODO: Pass error to monitor
                    Err(_) => return
                }

                match io_rx.try_recv() {
                    Ok(data) => {
                        serial.write(data.as_str());
                    },
                    Err(e) if e == mpsc::TryRecvError::Empty => (),
                    Err(_) => {
                        // TODO: Handle error
                    }
                }
            }
        });
    }

    // run ui
    match monitor.run() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("monitor")
        .about("Continuously display serial port data")
        .args(commands::serial_arguments(false, true).as_slice())
}
