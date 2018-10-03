/*
 * File: tests/test_case.rs
 * Date: 03.10.2018
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

use std::io;
use std::str;

use colored::*;

use serial::Serial;
use utils;

pub struct TestCaseSettings {
    pub input_format: utils::TextFormat,
    pub output_format: utils::TextFormat,
    pub ignore_case: bool
}

impl Default for TestCaseSettings {
    fn default() -> TestCaseSettings {
        TestCaseSettings {
            ignore_case: false,
            input_format: utils::TextFormat::Text,
            output_format: utils::TextFormat::Text
        }
    }
}

pub struct TestCase {
    pub settings: TestCaseSettings,

    name: String,
    input: String,
    raw_input: String,
    output: String,
    raw_output: String,
    response: String,
    successful: bool,
    executed: bool
}

impl TestCase {
    pub fn new(name: String, input: String, raw_input: String, output: String, raw_output: String) -> TestCase {
        TestCase {
            name,
            input,
            raw_input,
            output,
            raw_output,
            settings: Default::default(),
            response: "".to_string(),
            successful: false,
            executed: false
        }
    }

    pub fn new_with_settings(name: String, input: String, raw_input: String, output: String, raw_output: String, settings: TestCaseSettings) -> TestCase {
        TestCase {
            name,
            input,
            raw_input,
            output,
            raw_output,
            settings,
            response: "".to_string(),
            successful: false,
            executed: false
        }
    } 

    pub fn run(&mut self, serial: &mut Serial) -> Result<(), String> {
        match serial.write_format(&self.input, &self.settings.input_format) {
            Ok(_) => (),
            Err(e) => return Err(format!("Unable to write to serial port {}", e))
        };

        let mut response = String::new();

        loop {
            match serial.read() {
                Ok(bytes) => {
                    let mut new_text = match self.settings.output_format {
                        utils::TextFormat::Text => str::from_utf8(bytes).unwrap().to_string(),
                        _ => utils::radix_string(bytes, &self.settings.output_format)
                    };

                    if self.settings.ignore_case {
                        new_text = new_text.to_lowercase();
                    }

                    response.push_str(new_text.as_str());

                    if self.output == response {
                        break;
                    }

                    if self.output.starts_with(response.as_str()) == false {
                        break;
                    }
                },
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                    if response.len() == 0 {
                        return Err("Connection timed out".to_string());
                    }

                    break;
                },
                Err(e) => return Err(format!("Error while running test {}", e))
            }
        }

        self.response = response;
        self.executed = true;
        self.successful = self.response == self.output;

        Ok(())
    }

    pub fn is_successful(&self) -> bool {
        self.successful
    }

    fn title(&self) -> String {
        if self.name != "" {
            format!("{} \"{}\"", self.name, self.raw_input)
        } else {
            self.raw_input.clone()
        }
    }
}

impl ToString for TestCase {
    fn to_string(&self) -> String {
        if self.executed == false {
            return format!("{}", self.title());
        }

        if self.successful {
            format!("{}...{}", self.title(), "OK".green())
        } else {
            format!("{}...{}, expected '{}' but received '{}'", self.title(), "Failed".red(), self.raw_output, self.response)
        }
    }
}
