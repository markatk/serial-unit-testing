/*
 * File: src/monitor.rs
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

extern crate serialport;

use std::io::{self, Write};
use std::time::Duration;

use clap::{ArgMatches, App, SubCommand, Arg};
use self::serialport::SerialPortSettings;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let port_name = matches.value_of("port").unwrap();

    let settings = get_serial_port_settings(matches).unwrap();

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            let hex_mode = matches.is_present("hex");

            read(&mut port, hex_mode).unwrap();
        },
        Err(e) => return Err(format!("Error opening port {:?}", e))
    };

    Ok(())
}

pub fn command<'a>() -> App<'a, 'a> {
    let databits = [ "5", "6", "7", "8" ];
    let parity = [ "none", "even", "odd" ];
    let stopbits = [ "1", "2" ];
    let flowcontrols = [ "none", "software", "hardware" ];

    SubCommand::with_name("monitor")
        .about("Continously display serial port data")
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
        .arg(Arg::with_name("hex")
            .long("hex")
            .short("H")
            .help("Set hexadecimal mode"))
        .arg(Arg::with_name("flowcontrol")
            .long("flowcontrol")
            .short("f")
            .help("Serial port flow control mode")
            .possible_values(&flowcontrols)
            .default_value("none"))
}

fn read(port: &mut Box<serialport::SerialPort>, hex_mode: bool) -> Result<(), String> {
    let mut serial_buf: Vec<u8> = vec![0; 1000];
    let mut row_entries = 0;

    loop {
        match port.read(&mut serial_buf) {
            Ok(t) => {
                if hex_mode {
                    for b in &serial_buf[..t] {
                        print!("0x{:02X} ", b);

                        row_entries += 1;
                        if row_entries > 20 {
                            row_entries = 0;

                            println!();
                        }
                    }
                } else {
                    io::stdout().write_all(&serial_buf[..t]).unwrap();
                }

                io::stdout().flush().unwrap();
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => return Err(format!("{:?}", e))
        }
    }
}

fn get_serial_port_settings(matches: &ArgMatches) -> Result<SerialPortSettings, String> {
    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(1000);

    let baud_rate = matches.value_of("baud").unwrap();
    let data_bits = matches.value_of("databits").unwrap();
    let parity = matches.value_of("parity").unwrap();
    let stop_bits = matches.value_of("stopbits").unwrap();
    let flow_control = matches.value_of("flowcontrol").unwrap();

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

    settings.flow_control = match flow_control {
        "none" => serialport::FlowControl::None,
        "software" => serialport::FlowControl::Software,
        "hardware" => serialport::FlowControl::Hardware,
        _ => {
            return Err(format!("Invalid flow control '{}'", flow_control));
        }
    };

    Ok(settings)
}
