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
    pub ignore_case: bool,
    pub repeat: u32,
    pub delay: Option<Duration>,
    pub timeout: Duration
}

impl Default for TestCaseSettings {
    fn default() -> TestCaseSettings {
        TestCaseSettings {
            ignore_case: false,
            repeat: 0,
            delay: None,
            timeout: Duration::from_secs(1)
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

        if self.settings.ignore_case {
            output = output.to_lowercase();
        }

        let regex = Regex::new(&output).unwrap();

        // run test repeat + 1 times
        let mut success: bool = false;

        for _ in 0..self.settings.repeat + 1 {
            // if delay is set wait before execution
            if let Some(delay) = self.settings.delay {
                sleep(delay);
            }

            match serial.write_format(&input, &self.input_format) {
                Ok(_) => (),
                Err(e) => return Err(format!("Unable to write to serial port {}", e))
            };

            let mut response = String::new();

            loop {
                match serial.read_with_timeout(self.settings.timeout) {
                    Ok(bytes) => {
                        let mut new_text = match self.output_format {
                            utils::TextFormat::Text => str::from_utf8(bytes).unwrap().to_string(),
                            _ => utils::radix_string(bytes, &self.output_format)
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

                if self.settings.repeat > 0 {
                    repeat = format!(" ({}x)", self.settings.repeat);
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
