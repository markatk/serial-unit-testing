/*
 * File: tests/test_suite.rs
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

use crate::serial::Serial;

pub use crate::tests::test_case::{TestCase, TestCaseSettings};

/// Settings for running a test suite.
#[derive(Debug, Clone)]
pub struct TestSuiteSettings {
    /// If set the test suite will stop on first test failing.
    pub stop_on_failure: bool,
    /// If set the test suites will not be run.
    pub disabled: bool
}

impl Default for TestSuiteSettings {
    fn default() -> TestSuiteSettings {
        TestSuiteSettings {
            stop_on_failure: false,
            disabled: false
        }
    }
}

/// Test suite representing a group of tests.
#[derive(Debug)]
pub struct TestSuite {
    /// Name of the test group.
    pub name: String,
    /// Settings to use for the test suite.
    pub settings: TestSuiteSettings,
    /// Test settings to use for all tests.
    pub test_settings: TestCaseSettings,
    tests: Vec<TestCase>
}

impl TestSuite {
    /// Create a new test suite.
    pub fn new(name: String) -> TestSuite {
        TestSuite {
            name,
            settings: Default::default(),
            test_settings: Default::default(),
            tests: Vec::new()
        }
    }

    /// Create a new test suite with given suite and test settings.
    pub fn new_with_settings(name: String, settings: TestSuiteSettings, test_settings: TestCaseSettings) -> TestSuite {
        TestSuite {
            name,
            settings,
            test_settings,
            tests: Vec::new()
        }
    }

    /// Add a test to the suite.
    ///
    /// Tests added to the suite will be run when the suite is executed.
    pub fn push(&mut self, test: TestCase) {
        self.tests.push(test);

        if let Some(pushed_test) = self.tests.last_mut() {
            pushed_test.settings.merge_weak(&self.test_settings);
        }
    }

    /// Run all tests belonging to the test suite on given serial port.
    ///
    /// Execution will stop early if stop_on_failure is set and a test fails.
    pub fn run(&mut self, serial: &mut Serial) -> Result<bool, String> {
        if self.settings.disabled {
            return Ok(true);
        }

        for test in self.tests.iter_mut() {
            let result = test.run(serial)?;

            if self.settings.stop_on_failure && result == false {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Run all tests belonging to the test suite on given serial port and print the results.
    ///
    /// Execution will stop early if stop_on_failure is set and a test fails.
    pub fn run_and_print(&mut self, serial: &mut Serial, quiet: bool) -> bool {
        let show_title = self.name != "";

        if show_title && quiet == false {
            println!("{}", self.title());
        }

        if self.settings.disabled {
            return true;
        }

        for test in self.tests.iter_mut() {
            if show_title && quiet == false {
                print!("\t");
            }

            let result = match test.run(serial) {
                Ok(success) => success,
                Err(_) => false
            };

            if quiet == false || result == false {
                println!("{}", test.to_string());
            }

            if result != true && self.settings.stop_on_failure {
                return false;
            }
        }

        true
    }

    /// Get the number of tests belonging to the test suite.
    pub fn len(&self) -> usize {
        self.tests.len()
    }

    /// Check if any test belongs to the test suite.
    pub fn is_empty(&self) -> bool {
        self.tests.is_empty()
    }

    /// Get the number of failed tests.
    ///
    /// Will return 0 if not run before.
    pub fn failed(&self) -> usize {
        self.count_tests(false)
    }

    /// Get the number of successful tests.
    ///
    /// Will return 0 if not run before.
    pub fn successful(&self) -> usize {
        self.count_tests(true)
    }

    fn count_tests(&self, success: bool) -> usize {
        let mut count = 0;

        for test in &self.tests {
            if test.is_successful().is_none() && test.error().is_none() {
                continue;
            }

            if (test.is_successful().unwrap_or(false) || test.settings.allow_failure.unwrap_or(false)) == success {
                count += 1;
            }
        }

        count
    }

    fn title(&self) -> String {
        if self.settings.disabled {
            format!("{}: Disabled", self.name)
        } else {
            format!("{}:", self.name)
        }
    }
}

impl ToString for TestSuite {
    fn to_string(&self) -> String {
        let mut result = String::new();

        let show_group = self.name != "";

        if show_group {
            result.push_str(format!("{}\n", self.title()).as_str());
        }

        for test in &self.tests {
            if show_group {
                result.push('\t');
            }

            result.push_str(format!("{}\n", test.to_string()).as_str());
        }

        result
    }
}
