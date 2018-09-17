/*
 * File: src/send.rs
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

use std::io::{self, Write};

use clap::{ArgMatches};
use serialport::SerialPortSettings;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let port_name = matches.value_of("port").unwrap();

    let settings = get_serial_port_settings(matches).unwrap();

    let text = matches.value_of("text").unwrap();

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            send_text(&mut port, text).unwrap();

            if matches.is_present("response") {
                read_response(&mut port).unwrap();
            }
        },
        Err(error) => println!("Error opening port {:?}", error)
    };

    Ok(())
}

fn send_text(port: &mut Box<serialport::SerialPort>, text: &str) -> Result<(), String> {
    match port.write(text.as_bytes()) {
        Ok(_) => (),
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(error) => println!("Error sending text {:?}", error)
    }

    Ok(())
}

fn read_response(port: &mut Box<serialport::SerialPort>) -> Result<(), String> {
    let mut serial_buf: Vec<u8> = vec![0; 1000];

    match port.read(serial_buf.as_mut_slice()) {
        Ok(t) => {
            io::stdout().write_all(&serial_buf[..t]).unwrap();

            println!("");
        },
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => eprintln!("{:?}", e)
    }

    Ok(())
}

fn get_serial_port_settings(matches: &ArgMatches) -> Result<SerialPortSettings, String> {
    let baud_rate = matches.value_of("baud").unwrap();
    let data_bits = matches.value_of("databits").unwrap();
    let parity = matches.value_of("parity").unwrap();
    let stop_bits = matches.value_of("stopbits").unwrap();
    // TODO: Add missing flow control

    let mut settings: SerialPortSettings = Default::default();

    if let Ok(rate) = baud_rate.parse::<u32>() {
        settings.baud_rate = rate.into();
    } else {
        return Err(format!("Invalid baud rate '{}'", baud_rate));
    }

    settings.data_bits = match data_bits {
        "5" => serialport::DataBits::Five,
        "6" => serialport::DataBits::Six,
        "7" => serialport::DataBits::Seven,
        "8" => serialport::DataBits::Eight,
        _ => {
            return Err(format!("Invalid data bits '{}'", data_bits));
        }
    };

    settings.parity = match parity {
        "none" => serialport::Parity::None,
        "even" => serialport::Parity::Even,
        "odd" => serialport::Parity::Odd,
        _ => {
            return Err(format!("Invalid parity '{}'", parity));
        }
    };

    settings.stop_bits = match stop_bits {
        "1" => serialport::StopBits::One,
        "2" => serialport::StopBits::Two,
        _ => {
            return Err(format!("Invalid stop bits '{}", stop_bits));
        }
    };

    Ok(settings)
}
