/*
 * File: src/monitor/text_storage.rs
 * Date: 20.11.2019
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

use std::cmp::min;
use serial_unit_testing::utils::{self, TextFormat, NewlineFormat};

#[derive(Debug, Clone)]
pub struct TextStorage {
    input: String,
    pub input_format: TextFormat,
    input_history: Vec<String>,
    input_history_index: i32,
    input_backup: String,
    cursor_position: usize,
    pub escape_input: bool,
    pub newline_format: NewlineFormat,

    output: String,
    pub output_format: TextFormat,
    pub output_line: usize,
    scroll_output: bool,
    last_output_size: usize
}

impl TextStorage {
    pub fn get_input(&self) -> String {
        self.input.clone()
    }

    pub fn is_input_empty(&self) -> bool {
        self.input.is_empty()
    }

    pub fn input_add(&mut self, c: char) {
        if self.cursor_position < utils::char_count(&self.input) {
            utils::insert_char(&mut self.input, self.cursor_position, c);
        } else {
            self.input.push(c);
        }

        self.advance_cursor();
    }

    pub fn output_add_str(&mut self, str: &str) {
        self.output.push_str(str);

        if self.scroll_output {
            self.output_line = self.output.lines().count();
        }
    }

    pub fn get_cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn reset_input(&mut self) {
        self.input.clear();

        self.cursor_position = 1;
        self.input_history_index = -1;
        self.input_backup.clear();
    }

    pub fn reset_output(&mut self) {
        self.output.clear();

        self.output_line = 0;
    }

    pub fn advance_cursor(&mut self) {
        if self.cursor_position < utils::char_count(&self.input) {
            self.cursor_position += 1;
        }
    }

    pub fn retreat_cursor(&mut self) {
        if self.cursor_position > 1 {
            self.cursor_position -= 1;
        }
    }

    pub fn cursor_at_beginning(&mut self) {
        self.cursor_position = 0;
    }

    pub fn cursor_at_end(&mut self) {
        self.cursor_position = utils::char_count(&self.input);
    }

    pub fn advance_output(&mut self) {
        let lines = self.output.lines().count();

        if self.output_line < lines {
            self.output_line += 1;
        }

        if self.output_line == lines {
            self.scroll_output = true;
        }
    }

    pub fn retreat_output(&mut self) {
        if self.output_line > 1 {
            self.output_line -= 1;
        }

        self.scroll_output = false;
    }

    pub fn advance_output_page(&mut self) {
        let lines = self.output.lines().count();

        if self.output_line < lines {
            self.output_line += self.last_output_size;

            if self.output_line > lines {
                self.output_line = lines;
            }
        }

        if self.output_line == lines {
            self.scroll_output = true;
        }
    }

    pub fn retreat_output_page(&mut self) {
        if self.output_line > 1 {
            self.output_line -= self.last_output_size;

            if self.output_line < 1 {
                self.output_line = 1;
            }
        }

        self.scroll_output = false;
    }

    pub fn remove_character(&mut self, advance_cursor: bool) {
        let char_count = utils::char_count(&self.input);

        if self.input.is_empty() || (advance_cursor && self.cursor_position == 0) || (!advance_cursor && self.cursor_position >= char_count) {
            return;
        }

        let offset = match advance_cursor {
            true => 1,
            false => 0
        };

        if self.cursor_position < char_count {
            utils::remove_char(&mut self.input, self.cursor_position - offset);
        } else {
            self.input.pop();
        }

        if advance_cursor {
            self.retreat_cursor();
        }
    }

    pub fn add_history_entry(&mut self) {
        if let Some(last_input) = self.input_history.first() {
            if *last_input != self.input {
                self.input_history.insert(0, self.input.clone());
            }
        } else {
            self.input_history.insert(0, self.input.clone());
        }
    }

    pub fn advance_history(&mut self) {
        if self.input_history.is_empty() {
            return;
        }

        if self.input_history_index == -1 {
            self.input_backup = self.input.clone();
        }

        self.input_history_index += 1;
        let max_index = self.input_history.len() as i32;

        if self.input_history_index >= max_index {
            self.input_history_index = max_index - 1;
        }

        self.input = self.input_history[self.input_history_index as usize].clone();

        self.cursor_at_end();
    }

    pub fn retreat_history(&mut self) {
        if self.input_history_index == -1 {
            return;
        }

        self.input_history_index -= 1;
        if self.input_history_index < 0 {
            self.input_history_index = -1;
        }

        // update input with history or input backup
        if self.input_history_index >= 0 {
            self.input = self.input_history[self.input_history_index as usize].clone();
        } else {
            self.input = self.input_backup.clone();
        }

        self.cursor_at_end();
    }

    pub fn get_output_lines(&mut self, visible_lines: usize) -> (String, String) {
        let total_count = self.output.lines().count();

        if self.output_line < visible_lines {
            self.output_line = min(visible_lines, total_count);
        }

        let text = self.output
            .lines()
            .rev()
            .skip(total_count - self.output_line)
            .take(visible_lines)
            .fold("".to_string(), |current, line| {
                let mut result = line.to_string();
                result.push('\n');
                result.push_str(&current);

                result
            });

        let counter = if total_count > 0 {
            let start_line = if self.output_line > visible_lines {
                self.output_line - visible_lines + 1
            } else {
                1
            };

            format!("({}-{}/{})", start_line, self.output_line, total_count)
        } else {
            "(0-0/0)".to_string()
        };

        // safe number of visible lines for page scrolling
        self.last_output_size = visible_lines;

        (text, counter)
    }
}

impl Default for TextStorage {
    fn default() -> Self {
        TextStorage {
            input: String::new(),
            input_format: TextFormat::Text,
            input_history: vec!(),
            input_history_index: -1,
            input_backup: String::new(),
            cursor_position: 1,
            escape_input: false,
            newline_format: NewlineFormat::LineFeed,
            output: String::new(),
            output_format: TextFormat::Text,
            output_line: 1,
            scroll_output: true,
            last_output_size: 0
        }
    }
}
