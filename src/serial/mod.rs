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
use std::io::{self, Error};
use std::time::Duration;

use serialport;

use utils;

pub enum SerialSettingsDataBits {
    Five,
    Six,
    Seven,
    Eight
}

pub enum SerialSettingsParity {
    None,
    Even,
    Odd
}

pub enum SerialSettingsStopBits {
    One,
    Two
}

pub enum SerialSettingsFlowControl {
    None,
    Software,
    Hardware
}

pub struct SerialSettings {
    pub baud_rate: u32,
    pub timeout: u64,

    pub data_bits: SerialSettingsDataBits,
    pub parity: SerialSettingsParity,
    pub stop_bits: SerialSettingsStopBits,
    pub flow_control: SerialSettingsFlowControl
}

impl SerialSettings {
    fn to_serial_port_settings(&self) -> serialport::SerialPortSettings {
        let data_bits = match self.data_bits {
            SerialSettingsDataBits::Five => serialport::DataBits::Five,
            SerialSettingsDataBits::Six => serialport::DataBits::Six,
            SerialSettingsDataBits::Seven => serialport::DataBits::Seven,
            SerialSettingsDataBits::Eight => serialport::DataBits::Eight
        };

        let parity = match self.parity {
            SerialSettingsParity::None => serialport::Parity::None,
            SerialSettingsParity::Even => serialport::Parity::Even,
            SerialSettingsParity::Odd => serialport::Parity::Odd
        };

        let stop_bits = match self.stop_bits {
            SerialSettingsStopBits::One => serialport::StopBits::One,
            SerialSettingsStopBits::Two => serialport::StopBits::Two
        };

        let flow_control = match self.flow_control {
            SerialSettingsFlowControl::None => serialport::FlowControl::None,
            SerialSettingsFlowControl::Software => serialport::FlowControl::Software,
            SerialSettingsFlowControl::Hardware => serialport::FlowControl::Hardware
        };

        serialport::SerialPortSettings {
            baud_rate: self.baud_rate,
            timeout: Duration::from_millis(self.timeout),
            data_bits,
            parity,
            stop_bits,
            flow_control
        }
    }
}

impl Default for SerialSettings {
    fn default() -> SerialSettings {
        SerialSettings {
            baud_rate: 9600,
            timeout: 1000,
            data_bits: SerialSettingsDataBits::Eight,
            parity: SerialSettingsParity::None,
            stop_bits: SerialSettingsStopBits::One,
            flow_control: SerialSettingsFlowControl::None
        }
    }
}

pub struct Serial {
    port: Box<serialport::SerialPort>,
    read_buffer: Vec<u8>
}

impl Serial {
    pub fn open(port_name: &str) -> Result<Serial, String> {
        let settings: SerialSettings = Default::default();

        Serial::open_with_settings(port_name, &settings)
    }

    pub fn open_with_settings(port_name: &str, settings: &SerialSettings) -> Result<Serial, String> {
        match serialport::open_with_settings(&port_name, &settings.to_serial_port_settings()) {
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
