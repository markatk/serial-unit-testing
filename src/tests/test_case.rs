/*
 * File: tests/test_case.rs
 * Date: 03.10.2018
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

use std::io;
use std::str;
use std::time::Duration;
use std::thread::sleep;

use colored::*;
use regex::Regex;

use serial::Serial;
use utils;

#[derive(Debug)]
pub struct TestCaseSettings {
    pub ignore_case: Option<bool>,
    pub repeat: Option<u32>,
    pub delay: Option<Duration>,
    pub timeout: Option<Duration>
}

impl Default for TestCaseSettings {
    fn default() -> TestCaseSettings {
        TestCaseSettings {
            ignore_case: None,
            repeat: None,
            delay: None,
            timeout: None
        }
    }
}

#[derive(Debug)]
pub struct TestCase {
    pub settings: TestCaseSettings,
    pub input_format: utils::TextFormat,
    pub output_format: utils::TextFormat,

    name: String,
    input: String,
    output: String,
    response: Option<String>,
    successful: Option<bool>,
}

impl TestCase {
    pub fn new(name: String, input: String, output: String) -> TestCase {
        TestCase {
            name,
            input,
            output,
            settings: Default::default(),
            input_format: utils::TextFormat::Text,
            output_format: utils::TextFormat::Text,
            response: None,
            successful: None
        }
    }

    pub fn run(&mut self, serial: &mut Serial) -> Result<(), String> {
        // get input and desired output in correct format
        let input: String;
        let mut output: String;

        if self.input_format == utils::TextFormat::Text {
            input = self.descape_string(&self.input);
        } else {
            input = self.input.clone();
        }

        if self.output_format == utils::TextFormat::Text {
            output = self.descape_string(&self.output);
        } else {
            output = self.output.clone();
        }

        if self.settings.ignore_case.unwrap_or(false) {
            output = output.to_lowercase();
        }

        let regex = Regex::new(&output).unwrap();

        // run test repeat + 1 times
        let mut repeat = 1;
        let mut success: bool = false;

        if let Some(count) = self.settings.repeat {
            repeat += count;
        }

        for _ in 0..repeat {
            // if delay is set wait before execution
            if let Some(delay) = self.settings.delay {
                sleep(delay);
            }

            match serial.write_format(&input, &self.input_format) {
                Ok(_) => (),
                Err(e) => return Err(format!("Unable to write to serial port {}", e))
            };

            let response = self.read_response(serial)?;

            // check if response is correct
            if let Some(mat) = regex.find(&response) {
                success = mat.start() == 0 && mat.end() == response.len();
            } else {
                success = false;
            }

            self.response = Some(response);

            if success == false {
                break;
            }
        }

        self.successful = Some(success);

        Ok(())
    }

    pub fn is_successful(&self) -> Option<bool> {
        self.successful
    }

    fn read_response(&mut self, serial: &mut Serial) -> Result<String, String> {
        let mut response = String::new();

        loop {
            let response_chunk;

            if let Some(timeout) = self.settings.timeout {
                response_chunk = serial.read_with_timeout(timeout);
            } else {
                response_chunk = serial.read();
            }

            match response_chunk {
                Ok(bytes) => {
                    let mut new_text = match self.output_format {
                        utils::TextFormat::Text => str::from_utf8(bytes).unwrap().to_string(),
                        _ => utils::radix_string(bytes, &self.output_format)
                    };

                    if self.settings.ignore_case.unwrap_or(false) {
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

        Ok(response)
    }

    fn title(&self) -> String {
        if self.name != "" {
            format!("{} \"{}\"", self.name, self.input)
        } else {
            self.input.clone()
        }
    }

    fn descape_string(&self, text: &str) -> String {
        let mut response = String::new();
        let mut descape_next_char = false;
        let mut iterator = text.chars();

        loop {
            match iterator.next() {
                Some('t') if descape_next_char => response.push('\t'),
                Some('r') if descape_next_char => response.push('\r'),
                Some('n') if descape_next_char => response.push('\n'),
                Some('\\') if descape_next_char == false => {
                    descape_next_char = true;

                    continue;
                },
                Some(ch) => response.push(ch),
                None => break
            };

            descape_next_char = false;
        }

        response
    }
}

impl ToString for TestCase {
    fn to_string(&self) -> String {
        if let Some(successful) = self.successful {
            if successful {
                let mut repeat = String::new();

                if let Some(count) = self.settings.repeat {
                    repeat = format!(" ({}x)", count);
                }

                format!("{}...{}{}", self.title(), "OK".green(), repeat)
            } else {
                if let Some(ref response) = self.response {
                    format!("{}...{}, expected '{}' but received '{}'", self.title(), "Failed".red(), self.output, response)
                } else {
                    format!("{}...{}, expected '{}' but received nothing", self.title(), "Failed".red(), self.output)
                }
            }
        } else {
            format!("{}", self.title())
        }
    }
}
