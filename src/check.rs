/*
 * File: src/check.rs
 * Date: 30.09.2018
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

use std::io;

use clap::{ArgMatches, SubCommand, Arg, App};

use serial_unit_testing::utils;
use serial_unit_testing::serial::{Serial, CheckSettings};

use commands;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let (settings, port_name) = commands::get_serial_settings(matches).unwrap();

    let mut serial = Serial::open_with_settings(port_name, &settings)?;

    let mut text = matches.value_of("text").unwrap().to_string();
    let response = matches.value_of("response").unwrap();

    let echo_text = matches.is_present("echo");
    let ignore_case = matches.is_present("ignorecase");

    let input_format = commands::get_text_input_format(matches);
    let output_format = commands::get_text_output_format(matches);

    if matches.is_present("newline") {
        text.push_str("\n");
    }

    if matches.is_present("carriagereturn") {
        text.push_str("\r");
    }

    if matches.is_present("escape") {
        text = utils::escape_text(text);
    }

    let check_settings = CheckSettings {
        ignore_case,
        input_format,
        output_format
    };

    let (result, actual_response) = match serial.check_with_settings(&text, &response, &check_settings) {
        Ok((result, actual_response)) => (result, actual_response),
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => return Err("Serial connection timed out".to_string()),
        Err(e) => return Err(format!("Error running check {:?}", e))
    };

    if echo_text {
        println!("{}", text);
    }

    if result {
        println!("OK");
    } else {
        println!("Mismatch: '{}' does not match '{}'", response, actual_response);
    }

    Ok(())
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("check")
        .about("Send data to serial port and check for correct response")
        .args(commands::serial_arguments().as_slice())
        .args(commands::text_input_arguments().as_slice())
        .args(commands::text_output_arguments().as_slice())
        .arg(Arg::with_name("echo")
            .long("echo")
            .short("e")
            .help("Echo send text to standard output"))
        .arg(Arg::with_name("ignorecase")
            .long("ignore-case")
            .short("c")
            .help("Ignore response letter case while comparing"))
        .arg(Arg::with_name("text")
            .help("Text send to the serial port")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("response")
            .help("Response to be checked against")
            .takes_value(true)
            .required(true))
}
