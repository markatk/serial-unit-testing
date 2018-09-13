/*
 * File: src/main.rs
 * Date: 12.09.2018
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

extern crate clap;
extern crate serialport;

use clap::{App, ArgMatches, SubCommand, Arg};
use serialport::{SerialPortInfo};

fn run(matches: ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("send", Some(m)) => run_send(m),
        ("list", Some(m)) => run_list(m),
        ("monitor", Some(m)) => run_monitor(m),
        _ => Ok(())
    }
}

pub fn run_send(matches: &ArgMatches) -> Result<(), String> {
    let _port_name = matches.value_of("port").unwrap();

    Ok(())
}

fn run_list(_matches: &ArgMatches) -> Result<(), String> {
    let ports: Vec<SerialPortInfo> = serialport::available_ports().unwrap();

    for port in ports.iter() {
        println!("{}", port.port_name);
    }

    Ok(())
}

fn run_monitor(_matches: &ArgMatches) -> Result<(), String> {
    Ok(())
}

fn main() {
    let databits = [ "6", "7", "8", "9" ];
    let parity = [ "none", "even", "odd" ];
    let stopbits = [ "1", "2" ];

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
            .default_value("1"));

    let list_subcommand = SubCommand::with_name("list")
        .about("List all available serial ports");

    let matches = App::new("serial-unit-testing")
        .version("v0.1")
        .version_short("v")
        .about("Serial unit testing framework")
        .subcommand(send_subcommand)
        .subcommand(list_subcommand)
        .get_matches();

    if let Err(e) = run(matches) {
        println!("Application error: {}", e);

        return;
    }
}
