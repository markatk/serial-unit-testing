/*
 * File: src/list.rs
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

use clap::{ArgMatches, App, SubCommand, Arg};
use serialport::{self, SerialPortType};

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let verbose = matches.is_present("verbose");

    let ports = serialport::available_ports().unwrap();

    for port in ports {
        println!("{}", port.port_name);

        if verbose == false {
            continue;
        }
        
        match port.port_type {
            SerialPortType::UsbPort(info) => {
                println!("  Type: USB");
                println!("  VID: {:04x}", info.vid);
                println!("  PID: {:04x}", info.pid);
                println!("  Serial Number: {}", info.serial_number.as_ref().map_or("", String::as_str));
                println!("  Manufacturer: {}", info.manufacturer.as_ref().map_or("", String::as_str));
                println!("  Product: {}", info.product.as_ref().map_or("", String::as_str));
            }
            SerialPortType::BluetoothPort => {
                println!("  Type: Bluetooth");
            }
            SerialPortType::PciPort => {
                println!("  Type: PCI");
            }
            SerialPortType::Unknown => {
                println!("  Type: Unknown");
            }
        }

        println!("");
    }

    Ok(())
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("list")
        .about("List all available serial ports")
        .arg(Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .help("Print detailed information about each serial port"))
}
