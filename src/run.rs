/*
 * File: src/run.rs
 * Date: 01.10.2018
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
use colored::*;
use serial_unit_testing::serial::Serial;
use serial_unit_testing::parser;
use serial_unit_testing::tests::TestCaseSettings;
use crate::commands;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let filename = matches.value_of("file").unwrap();

    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => return Err("File not found".to_string()),
        Err(e) => return Err(format!("{}", e))
    };

    // open serial
    let (settings, port_name) = commands::get_serial_settings(matches).unwrap();

    let mut serial = match Serial::open_with_settings(port_name, settings) {
        Ok(serial) => serial,
        Err(e) => return Err(format!("Error opening port {:?}", e))
    };

    // parse and run tests
    let default_test_settings = TestCaseSettings {
        verbose: Some(matches.is_present("verbose")),
        ..Default::default()
    };

    let test_suites = match parser::parse_file_with_default_settings(&mut file, default_test_settings) {
        Ok(test_suites) => test_suites,
        Err(e) => return Err(format!("Unable to parse file: {}", e))
    };

    let mut total_tests = 0;
    let mut successful_tests = 0;
    let mut failed_tests = 0;

    let stop_on_failure = matches.is_present("stop");
    let quiet = matches.is_present("quiet");

    for mut test_suite in test_suites {
        // only set stop_on_failure and never reset
        if stop_on_failure {
            test_suite.settings.stop_on_failure = stop_on_failure;
        }

        let result = test_suite.run_and_print(&mut serial, quiet);

        let successful = test_suite.successful();
        let failed = test_suite.failed();

        total_tests += successful + failed;
        successful_tests += successful;
        failed_tests += failed;

        if !quiet {
            println!();
        }

        if !result && test_suite.settings.stop_on_failure {
            // global stop on failure?
            if stop_on_failure {
                println!("Stopping because 'stop-on-failure' is set");

                break;
            } else {
                println!("Stopping group because 'stop-on-failure' is set\n");

                continue;
            }
        }
    }

    if !quiet {
        println!("\nRan {} tests, {} successful, {} failed", total_tests.to_string().yellow(), successful_tests.to_string().green(), failed_tests.to_string().red());
    }

    Ok(())
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("run")
        .about("Run script on serial port")
        .arg(Arg::with_name("file")
            .help("Script to run on the serial port")
            .required(true)
            .takes_value(true))
        .args(commands::serial_arguments(true, false).as_slice())
        .arg(Arg::with_name("stop")
            .long("stop-on-failure")
            .short("S")
            .help("Stop on first test failing"))
        .arg(Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .help("Show verbose output"))
        .arg(Arg::with_name("quiet")
            .long("quiet")
            .short("q")
            .help("Only show failed tests"))
}
