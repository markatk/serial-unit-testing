/*
 * File: src/verify.rs
 * Date: 02.10.2018
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

use std::io;
use std::fs::File;

use clap::{ArgMatches, SubCommand, Arg, App};

use serial_unit_testing::parser;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let filename = matches.value_of("file").unwrap();

    let mut file = match File::open(filename) {
        Ok(mut file) => file,
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => return Err("File not found".to_string()), 
        Err(e) => return Err(format!("{}", e))
    };

    let test_suites = match parser::parse_file(&mut file) {
        Ok(test_suites) => test_suites,
        Err(e) => return Err(format!("Unable to parse file: {}", e))
    };

    match matches.occurrences_of("verbose") {
        1 => {
            for test_suite in test_suites {
                println!("Test suite '{}' with {} tests", test_suite.name, test_suite.len());
            }
        },
        0 => println!("OK"),
        _ => {
            for test_suite in test_suites {
                println!("{}", test_suite.to_string());
            }
        }
    };

    Ok(())
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("verify")
        .about("Verify a script can be parsed")
        .arg(Arg::with_name("file")
            .help("Script to verify")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .help("Show verbose output")
            .multiple(true))
}
