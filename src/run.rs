/*
 * File: src/run.rs
 * Date: 01.10.2018
 * Auhtor: Markus Grigull
 * 
 * MIT License
 * 
 * Copyright (c) 2018 Markus Grigull
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

use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::str;
use std::iter;

use clap::{ArgMatches, SubCommand, Arg, App};
use colored::*;

use serialunittesting::serial::{Serial, CheckSettings};
use serialunittesting::utils;

use commands;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let filename = matches.value_of("file").unwrap();

    let mut file = match File::open(filename) {
        Ok(mut file) => file,
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => return Err("File not found".to_string()), 
        Err(e) => return Err(format!("{}", e))
    };

    // open serial
    let (settings, port_name) = commands::get_serial_settings(matches).unwrap();

    let mut serial = Serial::open_with_settings(port_name, &settings)?;

    parse_file(&mut file, &mut serial)
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("run")
        .about("Run script on serial port")
        .args(commands::serial_arguments().as_slice())
        .arg(Arg::with_name("file")
            .help("Script to run on the serial port")
            .required(true)
            .takes_value(true))
}

fn parse_file(file: &mut File, serial: &mut Serial) -> Result<(), String> {
    let reader = BufReader::new(file);

    let mut total_tests = 0;
    let mut successful_tests = 0;
    let mut failed_tests = 0;

    for (num, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        let mut iterator = line.chars();
        let mut skip_line = false;

        loop {
            match iterator.next().unwrap() {
                ' ' | '\t' => (),
                '#' => {
                    skip_line = true;

                    break;
                },
                _ => break
            };
        }

        if skip_line {
            continue;
        }

        match execute_line(line.as_str(), serial) {
            Ok((result, message, desired_response, response)) => {
                print!("{}...", message);

                if result {
                    println!("{}", "OK".green());

                    successful_tests += 1;
                } else {
                    println!("{}, Expected '{}' but received '{}'", "Failed".red(), desired_response, response);

                    failed_tests += 1;
                }
            },
            Err(e) => println!("Error in line {}: {}", num, e)
        }

        total_tests += 1;
    }

    println!("\nRan {} tests, {} successful, {} failed", total_tests, successful_tests, failed_tests);

    Ok(())
}

fn execute_line(line: &str, serial: &mut Serial) -> Result<(bool, String, String, String), String> {
    let mut iterator: iter::Peekable<str::Chars> = line.chars().peekable();

    let input_format = get_text_format(&mut iterator)?;
    let (input, raw_input) = get_formatted_text(&mut iterator, &input_format)?;

    // skip separator
    let mut found_separator = false;

    loop {
        match iterator.peek().unwrap() {
            ' ' | '\t' => (),
            ':' if found_separator == false => {
                found_separator = true;

            },
            _ if found_separator => break,
            ch => return Err(format!("Unallowed character '{}'", ch))
        }

        iterator.next();
    }

    let output_format = get_text_format(&mut iterator)?;
    let (output, raw_output) = get_formatted_text(&mut iterator, &output_format)?;

    let settings = CheckSettings {
        ignore_case: false,
        input_format,
        output_format
    };

    match serial.check_with_settings(&input, &output, &settings) {
        Ok((result, response)) => Ok((result, raw_input, raw_output, response)),
        Err(e) => Err(format!("Error while executing check {}", e))
    }
}

fn get_text_format(iterator: &mut iter::Peekable<str::Chars>) -> Result<utils::TextFormat, String> {
    let first_char = iterator.peek().unwrap();

    match first_char {
        'b' => Ok(utils::TextFormat::Binary),
        'o' => Ok(utils::TextFormat::Octal),
        'd' => Ok(utils::TextFormat::Decimal),
        'h' => Ok(utils::TextFormat::Hex),
        '"' => Ok(utils::TextFormat::Text),
        _ => Err(format!("Unknown text format specifier '{}'", first_char))
    }
}

fn get_formatted_text(iterator: &mut iter::Peekable<str::Chars>, text_format: &utils::TextFormat) -> Result<(String, String), String> {
    if *text_format != utils::TextFormat::Text {
        iterator.next();
    }

    if iterator.next().unwrap() != '"' {
        return Err("Line is not in correct format".to_string());
    }

    let mut text = String::new();
    let mut raw_text = String::new();
    let mut escape_next_char = false;

    loop {
        let mut next_char = iterator.next().unwrap();

        if (escape_next_char == false && next_char == '"') == false{
            raw_text.push(next_char);
        }

        if escape_next_char {
            next_char = match next_char {
                't' => '\t',
                'n' => '\n',
                'r' => '\r',
                _ => next_char
            }
        } else {
            match next_char {
                '"' => break,
                '\\' if *text_format == utils::TextFormat::Text => {
                    escape_next_char = true;

                    continue;
                },
                _ => ()
            };
        }

        escape_next_char = false;

        text.push(next_char);
    }

    Ok((text, raw_text))
}
