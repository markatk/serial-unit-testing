/*
 * File: src/monitor.rs
 * Date: 13.09.2018
 * Auhtor: MarkAtk
 * 
 * MIT License
 * 
 * Copyright (c) 2018 MarkAtk
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

use std::io::{self, Write};

use clap::{ArgMatches, App, SubCommand};
use serialport;

use commands;
use utils;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let (settings, port_name) = commands::get_serial_port_settings(matches).unwrap();

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            let text_format = commands::get_text_output_format(matches);

            read(&mut port, text_format)
        },
        Err(e) => return Err(format!("Error opening port {:?}", e))
    }
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("monitor")
        .about("Continously display serial port data")
        .args(commands::serial_arguments().as_slice())
}

fn read(port: &mut Box<serialport::SerialPort>, text_format: utils::TextFormat) -> Result<(), String> {
    let mut serial_buf: Vec<u8> = vec![0; 1000];
    let mut row_entries = 0;

    loop {
        match port.read(&mut serial_buf) {
            Ok(t) => {
                match text_format {
                    utils::TextFormat::Text => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                    _ => utils::print_radix_string(&serial_buf[..t], &text_format, &mut row_entries)
                };

                io::stdout().flush().unwrap();
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => return Err(format!("{:?}", e))
        }
    }
}
