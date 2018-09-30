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
use std::str;

use clap::{ArgMatches, SubCommand, Arg, App};

use commands;
use utils;
use serial::Serial;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    match Serial::open(&matches) {
        Ok(mut serial) => {
            let mut text = matches.value_of("text").unwrap().to_string();
            let response = matches.value_of("response").unwrap();

            let echo_text = matches.is_present("echo");
            let ignore_case = matches.is_present("ignorecase");

            let input_text_format = commands::get_text_input_format(matches);
            let output_text_format = commands::get_text_output_format(matches);

            if matches.is_present("newline") {
                text.push_str("\n");
            }

            if matches.is_present("carriagereturn") {
                text.push_str("\r");
            }

            if matches.is_present("escape") {
                text = utils::escape_text(text);
            }

            send_text(&mut serial, text.as_str(), echo_text, &input_text_format).unwrap();

            check_response(&mut serial, &response, ignore_case, &output_text_format)
        },
        Err(e) => Err(e)
    }
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("check")
        .about("Send data to serial port and check for correct response")
        .args(commands::serial_arguments().as_slice())
        .args(commands::text_input_arguments().as_slice())
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

fn send_text(serial: &mut Serial, text: &str, echo_text: bool, text_format: &utils::TextFormat) -> Result<(), String> {
    match serial.write_format(text, &text_format) {
        Ok(_) => {
            if echo_text {
                println!("{}", text);
            }

            Ok(())
        },
        Err(e) => Err(e)
    }
}

fn check_response(serial: &mut Serial, desired_response: &str, ignore_case: bool, text_format: &utils::TextFormat) -> Result<(), String> {
    let mut response = String::new();

    loop {
        match serial.read() {
            Ok(bytes) => {
                let mut new_text = match text_format {
                    utils::TextFormat::Text => str::from_utf8(bytes).unwrap().to_string(),
                    _ => utils::radix_string(bytes, &text_format)
                };

                if ignore_case {
                    new_text = new_text.to_lowercase();
                }

                response.push_str(new_text.as_str());

                if desired_response == response {
                    break;
                }

                if desired_response.starts_with(response.as_str()) == false {
                    break;
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => return Err("Timeout".to_string()),
            Err(e) => return Err(format!("{:?}", e))
        }
    }

    if desired_response == response {
        println!("OK");
    } else {
        print_mismatch(desired_response, &response);
    }

    Ok(())
}

fn print_mismatch(desired_response: &str, response: &str) {
    println!("Mismatch: '{}' does not match '{}'", desired_response, response);
}
