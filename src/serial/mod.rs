/*
 * File: src/serial/mod.rs
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
use std::io;
use std::str;

use serialport;

use utils;

pub mod settings;

pub struct Serial {
    port: Box<serialport::SerialPort>,
    read_buffer: Vec<u8>
}

pub struct CheckSettings {
    pub ignore_case: bool,
    pub input_format: utils::TextFormat,
    pub output_format: utils::TextFormat
}

impl Default for CheckSettings {
    fn default() -> CheckSettings {
        CheckSettings {
            ignore_case: false,
            input_format: utils::TextFormat::Text,
            output_format: utils::TextFormat::Text
        }
    }
}

impl Serial {
    pub fn open(port_name: &str) -> Result<Serial, String> {
        let settings: settings::Settings = Default::default();

        Serial::open_with_settings(port_name, &settings)
    }

    pub fn open_with_settings(port_name: &str, settings: &settings::Settings) -> Result<Serial, String> {
        match serialport::open_with_settings(&port_name, &settings.to_serial_port_settings()) {
            Ok(port) => {
                Ok(Serial { port, read_buffer: vec![0; 1000] })
            },
            Err(e) => Err(format!("Error opening port {:?}", e))
        }
    }

    pub fn write(&mut self, text: &str) -> Result<(), io::Error> {
        self.port.write(text.as_bytes())?;

        Ok(())
    }

    pub fn write_format(&mut self, text: &str, text_format: &utils::TextFormat) -> Result<(), io::Error> {
        let bytes = match text_format {
            utils::TextFormat::Binary => utils::bytes_from_binary_string(text).unwrap(),
            utils::TextFormat::Hex => utils::bytes_from_hex_string(text).unwrap(),
            _ => {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(text.as_bytes());

                bytes
            }
        };

        self.port.write(bytes.as_slice())?;

        Ok(())
    }

    pub fn read<'a>(&'a mut self) -> Result<&'a [u8], io::Error> {
        let length = self.port.read(&mut self.read_buffer)?;

        Ok(&self.read_buffer[..length])
    }

    pub fn check(&mut self, text: &str, desired_response: &str) -> Result<(bool, String), io::Error> {
        let settings: CheckSettings = Default::default();

        self.check_with_settings(text, desired_response, &settings)
    }

    pub fn check_with_settings(&mut self, text: &str, desired_response: &str, settings: &CheckSettings) -> Result<(bool, String), io::Error> {
        self.write_format(text, &settings.input_format)?;

        let mut response = String::new();

        loop {
            match self.read() {
                Ok(bytes) => {
                    let mut new_text = match settings.output_format {
                        utils::TextFormat::Text => str::from_utf8(bytes).unwrap().to_string(),
                        _ => utils::radix_string(bytes, &settings.output_format)
                    };

                    if settings.ignore_case {
                        new_text = new_text.to_lowercase();
                    }

                    response.push_str(new_text.as_str());

                    if desired_response == response {
                        break;
                    }

                    if desired_response.starts_with(response.as_str()) == false {
                        break;
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                    if response.len() == 0 {
                        return Err(io::Error::new(io::ErrorKind::TimedOut, "Connection timed out"));
                    }

                    break;
                },
                Err(e) => return Err(e)
            }
        }

        Ok((desired_response == response, response))
    }
}