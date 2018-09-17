/*
 * File: src/main.rs
 * Date: 12.09.2018
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

#[macro_use]
extern crate clap;
extern crate serialport;

use clap::{App, ArgMatches, SubCommand, Arg};

mod list;
mod send;
mod monitor;

fn run(matches: ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("send", Some(m)) => send::run(m),
        ("list", Some(m)) => list::run(m),
        ("monitor", Some(m)) => monitor::run(m),
        _ => Ok(())
    }
}

fn main() {
    let databits = [ "5", "6", "7", "8" ];
    let parity = [ "none", "even", "odd" ];
    let stopbits = [ "1", "2" ];
    let flowcontrols = [ "none", "software", "hardware" ];

    let send_subcommand = SubCommand::with_name("send")
        .about("Send data to serial port")
        .arg(Arg::with_name("port")
            .long("port")
            .short("p")
            .help("Serial port OS specific name")
            .required(true)
            .value_name("PORT")
            .takes_value(true))
        .arg(Arg::with_name("baud")
            .long("baud")
            .short("b")
            .help("Serial port baud rate")
            .takes_value(true)
            .default_value("9600"))
        .arg(Arg::with_name("databits")
            .long("databits")
            .short("d")
            .help("Serial port number of data bits")
            .takes_value(true)
            .possible_values(&databits)
            .default_value("8"))
        .arg(Arg::with_name("parity")
            .long("parity")
            .short("P")
            .help("Serial port parity")
            .takes_value(true)
            .possible_values(&parity)
            .default_value("none"))
        .arg(Arg::with_name("stopbits")
            .long("stopbits")
            .short("s")
            .help("Serial port stop bits")
            .takes_value(true)
            .possible_values(&stopbits)
            .default_value("1"))
        .arg(Arg::with_name("flowcontrol")
            .long("flowcontrol")
            .short("f")
            .help("Serial port flow control mode")
            .possible_values(&flowcontrols)
            .default_value("none"))
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
            .takes_value(true));

    let list_subcommand = SubCommand::with_name("list")
        .about("List all available serial ports")
        .arg(Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .help("Print detailed information about each serial port"));

    let matches = App::new("serial-unit-testing")
        .version(crate_version!())
        .version_short("v")
        .about("Serial unit testing framework")
        .subcommand(send_subcommand)
        .subcommand(list_subcommand)
        .get_matches();

    if let Err(e) = run(matches) {
        println!("Error: {}", e);

        return;
    }
}
