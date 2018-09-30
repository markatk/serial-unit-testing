/*
 * File: src/serial.rs
 * Date: 30.09.2018
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

use std::boxed::Box;
use std::io::{self, Error};

use serialport::{self, SerialPort};
use clap::ArgMatches;

use commands;
use utils;

pub struct Serial {
    port: Box<SerialPort>,
    read_buffer: Vec<u8>
}

impl Serial {
    pub fn open(matches: &ArgMatches) -> Result<Serial, String> {
        let (settings, port_name) = commands::get_serial_port_settings(matches).unwrap();

        match serialport::open_with_settings(&port_name, &settings) {
            Ok(port) => {
                Ok(Serial { port, read_buffer: vec![0; 1000] })
            },
            Err(e) => Err(format!("Error opening port {:?}", e))
        }
    }

    pub fn _write(&mut self, text: &str) -> Result<(), String> {
        match self.port.write(text.as_bytes()) {
            Ok(_) => Ok(()),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => Ok(()),
            Err(e) => Err(format!("Error sending text {:?}", e))
        }
    }

    pub fn write_format(&mut self, text: &str, text_format: &utils::TextFormat) -> Result<(), String> {
        let bytes = match text_format {
            utils::TextFormat::Binary => utils::bytes_from_binary_string(text).unwrap(),
            utils::TextFormat::Hex => utils::bytes_from_hex_string(text).unwrap(),
            _ => {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(text.as_bytes());

                bytes
            }
        };

        match self.port.write(bytes.as_slice()) {
            Ok(_) => Ok(()),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => Ok(()),
            Err(e) => Err(format!("Error sending text {:?}", e))
        }
    }

    pub fn read<'a>(&'a mut self) -> Result<&'a [u8], Error> {
        match self.port.read(&mut self.read_buffer) {
            Ok(length) => Ok(&self.read_buffer[..length]),
            Err(e) => Err(e)
        }
    }
}
