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

use serial::Serial;

pub use tests::test_case::{TestCase, TestCaseSettings};

#[derive(Debug, Clone)]
pub struct TestSuiteSettings {
    pub stop_on_failure: bool
}

impl Default for TestSuiteSettings {
    fn default() -> TestSuiteSettings {
        TestSuiteSettings {
            stop_on_failure: true
        }
    }
}

#[derive(Debug)]
pub struct TestSuite {
    pub name: String,
    pub settings: TestSuiteSettings,
    pub test_settings: TestCaseSettings,
    tests: Vec<TestCase>
}

impl TestSuite {
    pub fn new(name: String) -> TestSuite {
        TestSuite {
            name,
            settings: Default::default(),
            test_settings: Default::default(),
            tests: Vec::new()
        }
    }

    pub fn push(&mut self, test: TestCase) {
        self.tests.push(test);

        if let Some(pushed_test) = self.tests.last_mut() {
            pushed_test.settings.merge_weak(&self.test_settings);
        }
    }

    pub fn run(&mut self, serial: &mut Serial) -> Result<bool, String> {
        for mut test in self.tests.iter_mut() {
            test.run(serial)?;

            if self.settings.stop_on_failure && test.is_successful() == Some(false) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn run_and_print(&mut self, serial: &mut Serial) -> bool {
        let show_title = self.name != "";

        if show_title {
            println!("{}", self.title());
        }

        for test in self.tests.iter_mut() {
            if show_title {
                print!("\t");
            }

            let result = match test.run(serial) {
                Ok(_) => test.is_successful(),
                Err(_) => Some(false)
            };

            println!("{}", test.to_string());

            if result != Some(true) && self.settings.stop_on_failure {
                return false;
            }
        }

        true
    }

    pub fn len(&self) -> usize {
        self.tests.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tests.is_empty()
    }

    pub fn failed(&self) -> usize {
        self.count_tests(false)
    }

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
        format!("{}:", self.name)
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
