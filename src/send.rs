/*
 * File: src/send.rs
 * Date: 13.09.2018
 * Author: MarkAtk
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
use serial_unit_testing::utils;
use serial_unit_testing::serial::Serial;
use crate::commands;
use serial_unit_testing::utils::TextFormat;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let (settings, port_name) = commands::get_serial_settings(matches).unwrap();

    let mut serial = match Serial::open_with_settings(port_name, settings) {
        Ok(serial) => serial,
        Err(e) => return Err(format!("Error opening port {:?}", e))
    };

    let mut text = matches.value_of("text").unwrap().to_string();
    let echo_text = matches.is_present("echo");

    let input_text_format = commands::get_text_input_format(matches);
    let output_text_format = commands::get_text_output_format(matches);
    let newline_format = commands::get_newline_format(matches);

    utils::add_newline(&mut text, input_text_format, newline_format);

    if matches.is_present("escape") && input_text_format == TextFormat::Text {
        text = utils::escape_text(text);
    }

    match serial.write_format(&text, input_text_format) {
        Ok(_) => (),
        Err(e) if e.is_timeout() => return Err("Serial connection timed out".to_string()),
        Err(e) => return Err(format!("Error sending text {:?}", e))
    };

    if echo_text {
        println!("{}", text);
    }

    if matches.is_present("response") {
        read_response(&mut serial, &output_text_format).unwrap();
    }

    Ok(())
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("send")
        .about("Send data to serial port")
        .args(commands::serial_arguments(false, true).as_slice())
        .args(commands::text_input_arguments().as_slice())
        .args(commands::text_output_arguments().as_slice())
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
            .required(true)
            .takes_value(true))
}

fn read_response(serial: &mut Serial, text_format: &utils::TextFormat) -> Result<(), String> {
    let mut row_entries = 0;

    loop {
        match serial.read() {
            Ok(bytes) => {
                match text_format {
                    utils::TextFormat::Text => io::stdout().write_all(bytes).unwrap(),
                    _ => utils::print_radix_string(bytes, text_format, &mut row_entries)
                };

                io::stdout().flush().unwrap();
            },
            Err(e) if e.is_timeout() => break,
            Err(e) => return Err(format!("{:?}", e))
        };
    }

    println!();

    Ok(())
}
