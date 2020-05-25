/*
 * File: src/serial/settings.rs
 * Date: 01.10.2018
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

use std::time::Duration;
use std::convert::From;

use serialport;

/// Number of bits for each data character.
#[derive(PartialEq, Clone, Eq, Copy, Debug)]
pub enum DataBits {
    /// 5 bits per character.
    Five,
    /// 6 bits per character.
    Six,
    /// 7 bits per character. Used for true ASCII.
    Seven,
    /// 8 bits per character. This is default in most cases.
    Eight
}

impl From<serialport::DataBits> for DataBits {
    fn from(data_bits: serialport::DataBits) -> Self {
        match data_bits {
            serialport::DataBits::Five => DataBits::Five,
            serialport::DataBits::Six => DataBits::Six,
            serialport::DataBits::Seven => DataBits::Seven,
            serialport::DataBits::Eight => DataBits::Eight
        }
    }
}

impl From<DataBits> for serialport::DataBits {
    fn from(data_bits: DataBits) -> Self {
        match data_bits {
            DataBits::Five => serialport::DataBits::Five,
            DataBits::Six => serialport::DataBits::Six,
            DataBits::Seven => serialport::DataBits::Seven,
            DataBits::Eight => serialport::DataBits::Eight
        }
    }
}

/// Data parity check modes.
#[derive(PartialEq, Clone, Eq, Copy, Debug)]
pub enum Parity {
    /// No parity bit used.
    None,
    /// Parity bit sets for even number of 1 bits.
    Even,
    /// Parity bit sets for odd number of 1 bits.
    Odd
}

impl From<serialport::Parity> for Parity {
    fn from(parity: serialport::Parity) -> Self {
        match parity {
            serialport::Parity::None => Parity::None,
            serialport::Parity::Even => Parity::Even,
            serialport::Parity::Odd => Parity::Odd
        }
    }
}

impl From<Parity> for serialport::Parity {
    fn from(parity: Parity) -> Self {
        match parity {
            Parity::None => serialport::Parity::None,
            Parity::Even => serialport::Parity::Even,
            Parity::Odd => serialport::Parity::Odd
        }
    }
}

/// Number of stop bits.
#[derive(PartialEq, Clone, Eq, Copy, Debug)]
pub enum StopBits {
    /// One stop bit.
    One,
    /// Two stop bits.
    Two
}

impl From<serialport::StopBits> for StopBits {
    fn from(stop_bits: serialport::StopBits) -> Self {
        match stop_bits {
            serialport::StopBits::One => StopBits::One,
            serialport::StopBits::Two => StopBits::Two
        }
    }
}

impl From<StopBits> for serialport::StopBits {
    fn from(stop_bits: StopBits) -> Self {
        match stop_bits {
            StopBits::One => serialport::StopBits::One,
            StopBits::Two => serialport::StopBits::Two
        }
    }
}

/// Flow control modes.
#[derive(PartialEq, Clone, Eq, Copy, Debug)]
pub enum FlowControl {
    /// No flow control.
    None,
    /// Flow control using ASCII XON/XOFF bytes.
    Software,
    /// Flow control using RTS/CTS or DTR/DSR signals.
    Hardware
}

impl From<serialport::FlowControl> for FlowControl {
    fn from(parity: serialport::FlowControl) -> Self {
        match parity {
            serialport::FlowControl::None => FlowControl::None,
            serialport::FlowControl::Software => FlowControl::Software,
            serialport::FlowControl::Hardware => FlowControl::Hardware
        }
    }
}

impl From<FlowControl> for serialport::FlowControl {
    fn from(parity: FlowControl) -> Self {
        match parity {
            FlowControl::None => serialport::FlowControl::None,
            FlowControl::Software => serialport::FlowControl::Software,
            FlowControl::Hardware => serialport::FlowControl::Hardware
        }
    }
}

/// Settings of a serial port connection.
#[derive(PartialEq, Clone, Eq, Copy, Debug)]
pub struct Settings {
    /// Baud rate in bits per second.
    pub baud_rate: u32,
    /// Timeout duration in milliseconds.
    pub timeout: u64,
    /// Number of data bits.
    pub data_bits: DataBits,
    /// Parity bit mode.
    pub parity: Parity,
    /// Number of stop bits.
    pub stop_bits: StopBits,
    /// Flow control mode.
    pub flow_control: FlowControl
}

impl Settings {
    pub fn to_short_string(&self) -> String {
        let data_bits = match self.data_bits {
            DataBits::Five => 5,
            DataBits::Six => 6,
            DataBits::Seven => 7,
            DataBits::Eight => 8
        };

        let parity = match self.parity {
            Parity::None => "N",
            Parity::Even => "E",
            Parity::Odd => "O"
        };

        let stop_bits = match self.stop_bits {
            StopBits::One => 1,
            StopBits::Two => 2
        };

        format!("{} {}{}{}", self.baud_rate, data_bits, parity, stop_bits)
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            baud_rate: 9600,
            timeout: 1000,
            data_bits: DataBits::Eight,
            parity: Parity::None,
            stop_bits: StopBits::One,
            flow_control: FlowControl::None
        }
    }
}

impl From<serialport::SerialPortSettings> for Settings {
    fn from(settings: serialport::SerialPortSettings) -> Self {
        Settings {
            baud_rate: settings.baud_rate,
            timeout: settings.timeout.as_millis() as u64,
            data_bits: settings.data_bits.into(),
            parity: settings.parity.into(),
            stop_bits: settings.stop_bits.into(),
            flow_control: settings.flow_control.into()
        }
    }
}

impl From<Settings> for serialport::SerialPortSettings {
    fn from(settings: Settings) -> Self {
        serialport::SerialPortSettings {
            baud_rate: settings.baud_rate,
            timeout: Duration::from_millis(settings.timeout),
            data_bits: settings.data_bits.into(),
            parity: settings.parity.into(),
            stop_bits: settings.stop_bits.into(),
            flow_control: settings.flow_control.into()
        }
    }
}
