/*
 * File: src/send.rs
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

use clap::{ArgMatches, SubCommand, Arg, App};
use serialport;

use commands;
use utils;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let port_name = matches.value_of("port").unwrap();

    let settings = commands::get_serial_port_settings(matches).unwrap();

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            let mut text = matches.value_of("text").unwrap().to_string();
            let echo_text = matches.is_present("echo");
            let hex_mode = matches.is_present("hex");
            let binary_mode = matches.is_present("binary");

            if matches.is_present("newline") {
                text.push_str("\n");
            }

            if matches.is_present("carriagereturn") {
                text.push_str("\r");
            }

            if matches.is_present("escape") {
                text = utils::escape_text(text);
            }

            send_text(&mut port, text.as_str(), echo_text, hex_mode, binary_mode).unwrap();

            if matches.is_present("response") {
                read_response(&mut port, hex_mode, binary_mode).unwrap();
            }
        },
        Err(e) => return Err(format!("Error opening port {:?}", e))
    };

    Ok(())
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("send")
        .about("Send data to serial port")
        .args(commands::serial_arguments().as_slice())
        .args(commands::text_input_arguments().as_slice())
        .arg(Arg::with_name("echo")
            .long("echo")
            .short("e")
            .help("Echo send text in standard output"))
        .arg(Arg::with_name("response")
            .long("show-response")
            .short("r")
            .help("Show response from device"))
        .arg(Arg::with_name("text")
            .help("Text send to the serial port")
            .takes_value(true))
}

fn send_text(port: &mut Box<serialport::SerialPort>, text: &str, echo_text: bool, hex_mode: bool, binary_mode: bool) -> Result<(), String> {
    let mut bytes: Vec<u8>;

    if hex_mode {
        bytes = utils::bytes_from_hex_string(text).unwrap();
    } else if binary_mode {
        bytes = utils::bytes_from_binary_string(text).unwrap();
    } else {
        bytes = Vec::new();
        bytes.extend_from_slice(text.as_bytes());
    }

    match port.write(bytes.as_slice()) {
        Ok(_) => {
            if echo_text {
                println!("{}", text);
            }
        },
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => return Err(format!("Error sending text {:?}", e))
    }

    Ok(())
}

fn read_response(port: &mut Box<serialport::SerialPort>, hex_mode: bool, binary_mode: bool) -> Result<(), String> {
    let mut serial_buf: Vec<u8> = vec![0; 1000];
    let mut row_entries = 0;

    loop {
        match port.read(&mut serial_buf) {
            Ok(t) => {
                if t <= 0 {
                    break;
                }

                if hex_mode {
                    utils::print_radix_string(&serial_buf[..t], 16, &mut row_entries, 20);
                } else if binary_mode {
                    utils::print_radix_string(&serial_buf[..t], 2, &mut row_entries, 10);
                } else {
                    io::stdout().write_all(&serial_buf[..t]).unwrap();
                }

                io::stdout().flush().unwrap();
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => break,
            Err(e) => return Err(format!("{:?}", e))
        }
    }

    println!();

    Ok(())
}
