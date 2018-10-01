/*
 * File: src/serial/settings.rs
 * Date: 01.10.2018
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

use std::time::Duration;

use serialport;

#[derive(PartialEq, Debug)]
pub enum DataBits {
    Five,
    Six,
    Seven,
    Eight
}

#[derive(PartialEq, Debug)]
pub enum Parity {
    None,
    Even,
    Odd
}

#[derive(PartialEq, Debug)]
pub enum StopBits {
    One,
    Two
}

#[derive(PartialEq, Debug)]
pub enum FlowControl {
    None,
    Software,
    Hardware
}

pub struct Settings {
    pub baud_rate: u32,
    pub timeout: u64,

    pub data_bits: DataBits,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub flow_control: FlowControl
}

impl Settings {
    pub fn to_serial_port_settings(&self) -> serialport::SerialPortSettings {
        let data_bits = match self.data_bits {
            DataBits::Five => serialport::DataBits::Five,
            DataBits::Six => serialport::DataBits::Six,
            DataBits::Seven => serialport::DataBits::Seven,
            DataBits::Eight => serialport::DataBits::Eight
        };

        let parity = match self.parity {
            Parity::None => serialport::Parity::None,
            Parity::Even => serialport::Parity::Even,
            Parity::Odd => serialport::Parity::Odd
        };

        let stop_bits = match self.stop_bits {
            StopBits::One => serialport::StopBits::One,
            StopBits::Two => serialport::StopBits::Two
        };

        let flow_control = match self.flow_control {
            FlowControl::None => serialport::FlowControl::None,
            FlowControl::Software => serialport::FlowControl::Software,
            FlowControl::Hardware => serialport::FlowControl::Hardware
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
