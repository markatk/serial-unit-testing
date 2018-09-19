/*
 * File: src/serial.rs
 * Date: 18.09.2018
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

struct SerialPort {
    port: serialport::SerialPort
}

impl SerialPort {
    fn write(&mut self, text: &str) -> Result<(), io::Error> {
        match self.port.write(text.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn read_available(&mut self) -> Result<String, io::Error> {
        let mut serial_buf: Vec<u8> = vec![0; 1000];
        let mut result = String::new();

        loop {
            match self.port.read(&mut serial_buf) {
                Ok(t) => {
                    if t <= 0 {
                        break;
                    }

                    let text = String::from_utf8(serial_buf[..t].to_vec()).unwrap();

                    result.push_str(&text);
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => break,
                Err(e) => println!("{}", e)
            };
        }

        Ok(result)
    }
}
