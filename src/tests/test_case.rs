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
use regex::Regex;

use serial::Serial;
use utils;


#[derive(Debug)]
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

#[derive(Debug)]
pub struct TestCase {
    pub settings: TestCaseSettings,

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
            response: None,
            successful: None
        }
    }

    pub fn new_with_settings(name: String, input: String, output: String, settings: TestCaseSettings) -> TestCase {
        TestCase {
            name,
            input,
            output,
            settings,
            response: None,
            successful: None
        }
    } 

    pub fn run(&mut self, serial: &mut Serial) -> Result<(), String> {
        let input: String;

        if self.settings.input_format == utils::TextFormat::Text {
            input = self.descape_string(&self.input);
        } else {
            input = self.input.clone();
        }

        match serial.write_format(&input, &self.settings.input_format) {
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

        let mut output: String;

        if self.settings.output_format == utils::TextFormat::Text {
            output = self.descape_string(&self.output);
        } else {
            output = self.output.clone();
        }

        if self.settings.ignore_case {
            output = output.to_lowercase();
        }

        let regex = Regex::new(&output).unwrap();

        if let Some(mat) = regex.find(&response) {
                    self.successful = Some(mat.start() == 0 && mat.end() == response.len());
        } else {
            self.successful = Some(false);
        }

        self.response = Some(response);

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
                format!("{}...{}", self.title(), "OK".green())
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
