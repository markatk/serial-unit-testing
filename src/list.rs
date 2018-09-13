/*
 * File: src/list.rs
 * Date: 13.09.2018
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

extern crate serialport;

use clap::{ArgMatches};
use serialport::SerialPortType;

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
