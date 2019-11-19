/*
 * File: src/serial/mod.rs
 * Date: 30.09.2018
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

use std::boxed::Box;
use std::str;
use std::time::Duration;
use serialport;
use crate::utils;
use crate::error::{Result, Error};

pub mod settings;
mod loopback;

use loopback::Loopback;

/// A serial port connection.
///
/// This struct handles the complete communication with a serial device regardless of the platform.
pub struct Serial {
    port: Box<dyn serialport::SerialPort>,
    read_buffer: Vec<u8>
}

impl Serial {
    /// Open a new connection with default settings.
    ///
    /// The port name is platform specific, e.g. starts with `COM` on Windows and `/dev/tty` or similar on UNIX systems.
    ///
    /// Default settings are:
    /// - Baud rate: 9600
    /// - Timeout: 1000 (ms)
    /// - Data bits: 8
    /// - Parity: None,
    /// - Stop bits: 1,
    /// - Flow control: None
    ///
    /// # Example
    ///
    /// ```
    /// use serial_unit_testing::serial::Serial;
    /// use serial_unit_testing::error::Result;
    ///
    /// fn main() -> Result<()> {
    ///     let mut serial = Serial::open("/dev/ttyACM0")?;
    ///     serial.write("Hello World!")?;
    ///
    ///     Ok(())
    /// }
    ///
    /// ```
    pub fn open(port_name: &str) -> Result<Serial> {
        let settings: settings::Settings = Default::default();

        Serial::open_with_settings(port_name, &settings)
    }

    /// Open a new connection with given settings.
    ///
    /// The port name is platform specific, e.g. starts with `COM` on Windows and `/dev/tty` or similar on UNIX systems.
    ///
    /// # Example
    ///
    /// ```
    /// use serial_unit_testing::serial::Serial;
    /// use serial_unit_testing::serial::settings::Settings;
    /// use serial_unit_testing::error::Result;
    ///
    /// fn main() -> Result<()> {
    ///     let mut settings = Settings::default();
    ///     settings.baud_rate = 115200;
    ///
    ///     let mut serial = Serial::open_with_settings("/dev/ttyACM0", &settings)?;
    ///     serial.write("Hello World!")?;
    ///
    ///     Ok(())
    /// }
    ///
    /// ```
    pub fn open_with_settings(port_name: &str, settings: &settings::Settings) -> Result<Serial> {
        if port_name == "loopback" {
            let port = Loopback::open(&settings.to_serial_port_settings());

            return Ok(Serial {
                port,
                read_buffer: vec![0; 1000]
            });
        }

        match serialport::open_with_settings(&port_name, &settings.to_serial_port_settings()) {
            Ok(port) => {
                Ok(Serial { port, read_buffer: vec![0; 1000] })
            },
            Err(e) => Err(Error::from(e))
        }
    }

    /// Write text to the serial port.
    ///
    /// This is the same as using `Serial::write_format` with `TextFormat::Text` as format specifier.
    pub fn write(&mut self, text: &str) -> Result<usize> {
        match self.port.write(text.as_bytes()) {
            Ok(count) => Ok(count),
            Err(e) => Err(Error::from(e))
        }
    }

    /// Write data in the given format.
    ///
    /// For a list of supported formats see `TextFormat`. `TextFormat::Text` is the same as using `Serial::write`.
    ///
    /// # Example
    ///
    /// ```
    /// use serial_unit_testing::serial::Serial;
    /// use serial_unit_testing::utils::TextFormat;
    /// use serial_unit_testing::error::Result;
    ///
    /// fn main() -> Result<()> {
    ///     let mut serial = Serial::open("/dev/ttyACM0")?;
    ///     serial.write_format("0a5f", TextFormat::Hex)?;
    ///
    ///     Ok(())
    /// }
    ///
    /// ```
    pub fn write_format(&mut self, text: &str, text_format: utils::TextFormat) -> Result<usize> {
        let bytes = match text_format {
            utils::TextFormat::Binary => utils::bytes_from_binary_string(text)?,
            utils::TextFormat::Octal => utils::bytes_from_octal_string(text)?,
            utils::TextFormat::Decimal => utils::bytes_from_decimal_string(text)?,
            utils::TextFormat::Hex => utils::bytes_from_hex_string(text)?,
            _ => {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(text.as_bytes());

                bytes
            }
        };

        match self.port.write(bytes.as_slice()) {
            Ok(count) => Ok(count),
            Err(e) => Err(Error::from(e))
        }
    }

    /// Read any amount of data.
    ///
    /// At least one byte of data must be read to return data. The method fails when no data could be read in the timeout duration.
    ///
    /// # Example
    ///
    /// ```
    /// use serial_unit_testing::serial::Serial;
    /// use serial_unit_testing::error::Result;
    ///
    /// fn main() -> Result<()> {
    ///     let mut serial = Serial::open("/dev/ttyACM0")?;
    ///     let data = serial.read().unwrap();
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn read(&mut self) -> Result<&[u8]> {
        let length = match self.port.read(&mut self.read_buffer) {
            Ok(length) => length,
            Err(e) => return Err(Error::from(e))
        };

        Ok(&self.read_buffer[..length])
    }

    /// Read a string.
    ///
    /// At least one character must be read to return successfully. The method fails when no characters could be read in the timeout duration.
    pub fn read_str(&mut self) -> Result<String> {
        self.read_str_with_format(utils::TextFormat::Text)
    }

    /// Read a string with minimum length.
    ///
    /// At least the amount of characters given by `min_length` must be read to return successfully. The method fails when no characters could be read in the timeout duration.
    pub fn read_min_str(&mut self, min_length: usize) -> Result<String> {
        self.read_min_str_with_format(min_length, utils::TextFormat::Text)
    }

    /// Read a string as given format.
    ///
    /// The bytes received will be formatted into the given string format.
    ///
    /// At least one character must be read to return successfully. The method fails when no characters could be read in the timeout duration.
    pub fn read_str_with_format(&mut self, format: utils::TextFormat) -> Result<String> {
        let data = self.read()?;

        utils::radix_string(data, &format)
    }

    /// Read a string as given format.
    ///
    /// The bytes received will be formatted into the given string format.
    ///
    /// At least the amount of characters (not bytes) given by `min_length` must be read to return successfully.
    /// The method fails when no characters could be read in the timeout duration.
    pub fn read_min_str_with_format(&mut self, min_length: usize, format: utils::TextFormat) -> Result<String> {
        let mut response = String::new();

        loop {
            match self.read() {
                Ok(bytes) => {
                    let new_text = utils::radix_string(bytes, &format)?;

                    response.push_str(new_text.as_str());

                    if response.len() >= min_length {
                        break;
                    }
                },
                Err(e) if e.is_timeout() => {
                    if response.len() == 0 {
                        return Err(e);
                    }

                    break;
                },
                Err(e) => return Err(e)
            }
        }

        Ok(response)
    }

    /// Read any amount of data in given timeout duration.
    ///
    /// This function can be used to use a different timeout for a single read. Otherwise see the timeout property of serial.
    ///
    /// At least one byte of data must be read to return data. The method fails when no data could be read in the timeout duration.
    pub fn read_with_timeout(&mut self, timeout: Duration) -> Result<&[u8]> {
        // remember old timeout
        let old_timeout = self.port.timeout();
        if let Err(e) = self.port.set_timeout(timeout) {
            return Err(Error::from(e));
        }

        let length = self.port.read(&mut self.read_buffer)?;

        if let Err(e) = self.port.set_timeout(old_timeout) {
            return Err(Error::from(e));
        }

        Ok(&self.read_buffer[..length])
    }

    /// Read a string in given timeout duration.
    ///
    /// This function can be used to use a different timeout for a single read. Otherwise see the timeout property of serial.
    ///
    /// At least one character must be read to return successfully. The method fails when no data could be read in the timeout duration.
    pub fn read_str_with_timeout(&mut self, timeout: Duration) -> Result<String> {
        // remember old timeout
        let old_timeout = self.port.timeout();
        if let Err(e) = self.port.set_timeout(timeout) {
            return Err(Error::from(e));
        }

        let length = self.port.read(&mut self.read_buffer)?;

        if let Err(e) = self.port.set_timeout(old_timeout) {
            return Err(Error::from(e));
        }

        match str::from_utf8(&self.read_buffer[..length]) {
            Ok(text) => Ok(text.to_string()),
            Err(e) => Err(Error::from(e))
        }
    }

    /// Read a string as given format in given timeout duration.
    ///
    /// The bytes received will be formatted into the given string format.
    /// This function can be used to use a different timeout for a single read. Otherwise see the timeout property of serial.
    ///
    /// At least one character must be read to return successfully. The method fails when no characters could be read in the timeout duration.
    pub fn read_str_with_format_and_timeout(&mut self, format: utils::TextFormat, timeout: Duration) -> Result<String> {
        // remember old timeout
        let old_timeout = self.port.timeout();
        if let Err(e) = self.port.set_timeout(timeout) {
            return Err(Error::from(e));
        }

        let length = self.port.read(&mut self.read_buffer)?;
        let data = &self.read_buffer[..length];

        if let Err(e) = self.port.set_timeout(old_timeout) {
            return Err(Error::from(e));
        }

        utils::radix_string(data, &format)
    }

    /// Send text to the serial and check if the response matches the desired response.
    ///
    /// The check will return early if the beginning of the responses does not match.
    ///
    /// Returns whether the actual response matches the desired response and the actual response. Fails with an timeout error or internal serial error.
    ///
    /// # Example
    ///
    /// ```
    /// use serial_unit_testing::serial::Serial;
    /// use serial_unit_testing::error::Result;
    ///
    /// fn main() -> Result<()> {
    ///     let mut serial = Serial::open("/dev/ttyACM0")?;
    ///     let (result, actual_response) = serial.check("hello", "world")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn check(&mut self, text: &str, desired_response: &str) -> Result<(bool, String)> {
        let settings: CheckSettings = Default::default();

        self.check_with_settings(text, desired_response, &settings)
    }

    /// Check if a response matches the desired response.
    ///
    /// The check will return early if the beginning of the responses does not match.
    ///
    /// Returns whether the actual response matches the desired response and the actual response. Fails with an timeout error or internal serial error.
    pub fn check_read(&mut self, desired_response: &str) -> Result<(bool, String)> {
        let settings: CheckSettings = Default::default();

        self.check_read_with_settings(desired_response, &settings)
    }

    /// Send text to the serial and check if the response matches the desired response with given settings.
    ///
    /// The check will return early if the beginning of the responses does not match.
    ///
    /// Returns whether the actual response matches the desired response and the actual response. Fails with an timeout error or internal serial error.
    pub fn check_with_settings(&mut self, text: &str, desired_response: &str, settings: &CheckSettings) -> Result<(bool, String)> {
        self.write_format(text, settings.input_format)?;

        self.check_read_with_settings(desired_response, settings)
    }

    /// Check if a response matches desired response with given settings.
    ///
    /// The check will return early if the beginning of the responses does not match.
    ///
    /// Returns whether the actual response matches the desired response and the actual response. Fails with an timeout error or internal serial error.
    pub fn check_read_with_settings(&mut self, desired_response: &str, settings: &CheckSettings) -> Result<(bool, String)> {
        let mut response = String::new();

        loop {
            match self.read() {
                Ok(bytes) => {
                    let mut new_text = utils::radix_string(bytes, &settings.output_format)?;

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
                Err(e) if e.is_timeout() => {
                    if response.len() == 0 {
                        return Err(e);
                    }

                    break;
                },
                Err(e) => return Err(e)
            }
        }

        Ok((desired_response == response, response))
    }
}

/// Settings for running tests on a serial port.
pub struct CheckSettings {
    /// Ignore response case mode.
    pub ignore_case: bool,
    /// Format of the data written to the serial port.
    pub input_format: utils::TextFormat,
    /// Format of the data received by the serial port.
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
