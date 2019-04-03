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
use crate::serial::Serial;
use crate::utils;

/// Settings for running a test.
///
/// All not set properties will be overwritten by the test suite.
#[derive(Debug, Clone)]
pub struct TestCaseSettings {
    /// Ignore case for string comparison.
    pub ignore_case: Option<bool>,
    /// Repeat the test given times.
    pub repeat: Option<u32>,
    /// Wait given milliseconds before executing the test.
    pub delay: Option<Duration>,
    /// Timeout duration before the test will fail.
    pub timeout: Option<Duration>,
    /// Allow the test to fail.
    pub allow_failure: Option<bool>,
    /// Print additional information when executing the test.
    pub verbose: Option<bool>
}

impl TestCaseSettings {
    /// Merge test settings with other test settings.
    ///
    /// Properties will be set if own property is not set but other's is.
    /// Own properties will not be overwritten.
    pub fn merge_weak(&mut self, other: &TestCaseSettings) {
        if self.ignore_case.is_none() && other.ignore_case.is_some() {
            self.ignore_case = other.ignore_case;
        }

        if self.repeat.is_none() && other.repeat.is_some() {
            self.repeat = other.repeat;
        }

        if self.delay.is_none() && other.delay.is_some() {
            self.delay = other.delay;
        }

        if self.timeout.is_none() && other.timeout.is_some() {
            self.timeout = other.timeout;
        }

        if self.allow_failure.is_none() && other.allow_failure.is_some() {
            self.allow_failure = other.allow_failure;
        }

        if self.verbose.is_none() && other.verbose.is_some() {
            self.verbose = other.verbose;
        }
    }
}

impl Default for TestCaseSettings {
    fn default() -> TestCaseSettings {
        TestCaseSettings {
            ignore_case: None,
            repeat: None,
            delay: None,
            timeout: None,
            allow_failure: None,
            verbose: None
        }
    }
}

/// Test representing a check on the serial.
#[derive(Debug)]
pub struct TestCase {
    /// Settings to use for the test.
    pub settings: TestCaseSettings,
    /// Text format of the input send to the serial.
    pub input_format: utils::TextFormat,
    /// Text format of the response received by the serial.
    pub output_format: utils::TextFormat,

    name: String,
    input: String,
    output: String,
    response: Option<String>,
    successful: Option<bool>,
    error: Option<String>
}

impl TestCase {
    /// Create a new test.
    pub fn new(name: String, input: String, output: String) -> TestCase {
        TestCase {
            name,
            input,
            output,
            settings: Default::default(),
            input_format: utils::TextFormat::Text,
            output_format: utils::TextFormat::Text,
            response: None,
            successful: None,
            error: None
        }
    }

    /// Execute the test on given serial port.
    ///
    /// After running the test response and successful are set if no error occurred.
    pub fn run(&mut self, serial: &mut Serial) -> Result<bool, String> {
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

        let regex = match Regex::new(&output) {
            Ok(regex) => regex,
            Err(_) => return self.exit_run_with_error(format!("Error in regex"))
        };

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
                Err(e) => return self.exit_run_with_error(format!("Unable to write to serial port: {}", e))
            };

            let response = match self.read_response(serial) {
                Ok(res) => res,
                Err(err) => return self.exit_run_with_error(err)
            };

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

        Ok(success)
    }

    /// Check if the test was successful.
    ///
    /// If the test was not run before None will be returned.
    pub fn is_successful(&self) -> Option<bool> {
        self.successful
    }

    /// Get the error from running the test.
    ///
    /// If the test was not run before or no error occurred None will be returned.
    pub fn error(&self) -> Option<String> {
        self.error.clone()
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

    fn exit_run_with_error(&mut self, err: String) -> Result<bool, String> {
        self.error = Some(err.clone());

        Err(err)
    }
}

impl ToString for TestCase {
    fn to_string(&self) -> String {
        if let Some(err) = &self.error {
            return format!("{}...{} {}", self.title(), "Error:".red(), err);
        }

        if let Some(successful) = self.successful {
            if successful == false && self.settings.allow_failure.unwrap_or(false) == false {
                return if let Some(ref response) = self.response {
                    format!("{}...{}, expected '{}' but received '{}'", self.title(), "Failed".red(), self.output, response)
                } else {
                    format!("{}...{}, expected '{}' but received nothing", self.title(), "Failed".red(), self.output)
                };
            }

            // test passed
            let repeat = if let Some(count) = self.settings.repeat {
                format!(" ({}x)", count)
            } else {
                String::new()
            };

            let verbose = if self.settings.verbose.unwrap_or(false) {
                if let Some(ref response) = self.response {
                    format!(", response: '{}'", response)
                } else {
                    format!(", no response")
                }
            } else {
                String::new()
            };

            let result = if successful {
                format!("{}", "OK".green())
            } else {
                format!("{} (failed)", "OK".yellow())
            };

            format!("{}...{}{}{}", self.title(), result, repeat, verbose)
        } else {
            format!("{}", self.title())
        }
    }
}
