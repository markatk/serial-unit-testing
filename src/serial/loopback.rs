/*
 * File: src/serial/loopback.rs
 * Date: 19.11.2019
 * Author: MarkAtk
 *
 * MIT License
 *
 * Copyright (c) 2019 MarkAtk
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
use std::io;
use std::thread;
use serialport::{self, SerialPort, StopBits, ClearBuffer, FlowControl, DataBits, Error, SerialPortSettings, Parity};

pub struct Loopback {
    settings: SerialPortSettings,
    buffer: Vec<u8>
}

impl Loopback {
    pub fn open(settings: SerialPortSettings) -> Box<Loopback> {
        Box::new(Loopback {
            settings,
            buffer: vec!()
        })
    }
}

impl SerialPort for Loopback {
    fn name(&self) -> Option<String> {
        Some("loopback".to_string())
    }

    fn settings(&self) -> SerialPortSettings {
        self.settings
    }

    fn baud_rate(&self) -> Result<u32, Error> {
        Ok(self.settings.baud_rate)
    }

    fn data_bits(&self) -> Result<DataBits, Error> {
        Ok(self.settings.data_bits)
    }

    fn flow_control(&self) -> Result<FlowControl, Error> {
        Ok(self.settings.flow_control)
    }

    fn parity(&self) -> Result<Parity, Error> {
        Ok(self.settings.parity)
    }

    fn stop_bits(&self) -> Result<StopBits, Error> {
        Ok(self.settings.stop_bits)
    }

    fn timeout(&self) -> Duration {
        self.settings.timeout
    }

    fn set_all(&mut self, settings: &SerialPortSettings) -> Result<(), Error> {
        self.settings = settings.clone();

        Ok(())
    }

    fn set_baud_rate(&mut self, baud_rate: u32) -> Result<(), Error> {
        self.settings.baud_rate = baud_rate;

        Ok(())
    }

    fn set_data_bits(&mut self, data_bits: DataBits) -> Result<(), Error> {
        self.settings.data_bits = data_bits;

        Ok(())
    }

    fn set_flow_control(&mut self, flow_control: FlowControl) -> Result<(), Error> {
        self.settings.flow_control = flow_control;

        Ok(())
    }

    fn set_parity(&mut self, parity: Parity) -> Result<(), Error> {
        self.settings.parity = parity;

        Ok(())
    }

    fn set_stop_bits(&mut self, stop_bits: StopBits) -> Result<(), Error> {
        self.settings.stop_bits = stop_bits;

        Ok(())
    }

    fn set_timeout(&mut self, timeout: Duration) -> Result<(), Error> {
        self.settings.timeout = timeout;

        Ok(())
    }

    fn write_request_to_send(&mut self, _level: bool) -> Result<(), Error> {
        // do nothing

        Ok(())
    }

    fn write_data_terminal_ready(&mut self, _level: bool) -> Result<(), Error> {
        // do nothing

        Ok(())
    }

    fn read_clear_to_send(&mut self) -> Result<bool, Error> {
        Ok(true)
    }

    fn read_data_set_ready(&mut self) -> Result<bool, Error> {
        Ok(true)
    }

    fn read_ring_indicator(&mut self) -> Result<bool, Error> {
        Ok(false)
    }

    fn read_carrier_detect(&mut self) -> Result<bool, Error> {
        Ok(false)
    }

    fn bytes_to_read(&self) -> Result<u32, Error> {
        Ok(self.buffer.len() as u32)
    }

    fn bytes_to_write(&self) -> Result<u32, Error> {
        // always return 0 because of loopback

        Ok(0)
    }

    fn clear(&self, _buffer_to_clear: ClearBuffer) -> Result<(), Error> {
        // do nothing (as buffer cannot be cleared without self being mutable)

        Ok(())
    }

    fn try_clone(&self) -> Result<Box<dyn SerialPort>, Error> {
        Ok(Box::new(Loopback {
            settings: self.settings.clone(),
            buffer: self.buffer.clone()
        }))
    }
}

impl io::Write for Loopback {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.buffer.extend_from_slice(buf);

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        // do nothing

        Ok(())
    }
}

impl io::Read for Loopback {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        if self.buffer.is_empty() {
            thread::sleep(self.settings.timeout);

            return Err(io::Error::new(io::ErrorKind::TimedOut, "Timed out"));
        }

        let len = self.buffer.len();

        for x in 0..len {
            buf[x] = self.buffer[x];
        }

        self.buffer.clear();

        Ok(len)
    }
}
