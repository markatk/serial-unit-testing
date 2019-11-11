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

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::error::Error;
use clap::{ArgMatches, App, SubCommand};
use crossterm::KeyEvent;
use crate::commands;
use crate::windows::{WindowManager, Event};
use serial_unit_testing::serial::Serial;

mod helpers;
mod main_window;
mod help_window;

use main_window::MainWindow;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let (io_tx, io_rx) = mpsc::channel();
    let (settings, port_name) = commands::get_serial_settings(matches).unwrap();

    // create windows
    let mut window_manager = match WindowManager::new() {
        Ok(window_manager) => window_manager,
        Err(e) => return Err(e.to_string())
    };

    let ui_tx = window_manager.get_tx().clone();

    // create main window
    let mut main_window = MainWindow::new(io_tx);

    main_window.input_format = commands::get_text_input_format(matches);
    main_window.output_format = commands::get_text_output_format(matches);
    main_window.newline_format = commands::get_newline_format(matches);
    main_window.title = format!("{}, {} ", port_name, settings.to_short_string());

    // open serial port
    let mut serial = match Serial::open_with_settings(port_name, &settings) {
        Ok(serial) => serial,
        Err(e) => return Err(format!("Unable to connect to port: {:?}", e.description()))
    };

    // start thread for receiving from and sending to serial port
    {
        thread::spawn(move || {
            loop {
                match serial.read_with_timeout(Duration::from_millis(10)) {
                    Ok(bytes) => {
                        if let Err(_) = ui_tx.send(Event::Output(bytes.to_vec())) {
                            eprintln!("Unable to send to ui thread");

                            return;
                        }
                    },
                    Err(e) if e.is_timeout() => (),
                    Err(_) => {
                        show_error(&ui_tx, "Unable to read from serial".to_string());

                        return;
                    }
                }

                match io_rx.try_recv() {
                    Ok((data, format)) => {
                        if let Err(_err) = serial.write_format(data.as_str(), format) {
                            show_error(&ui_tx, "Unable to write to serial".to_string());

                            return;
                        }
                    },
                    Err(e) if e == mpsc::TryRecvError::Empty => (),
                    Err(_) => {
                        show_error(&ui_tx, "I/O thread closed".to_string());

                        return;
                    }
                }
            }
        });
    }

    match window_manager.run(main_window) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("monitor")
        .about("Interactive serial communication monitor")
        .args(commands::serial_arguments(false, true).as_slice())
        .args(commands::text_input_arguments().as_slice())
        .args(commands::text_output_arguments().as_slice())
}

fn show_error(tx: &mpsc::Sender<Event<KeyEvent>>, text: String) {
    if let Err(_err) = tx.send(Event::Error(text)) {
        eprintln!("Unable to send to ui thread");
    }
}
