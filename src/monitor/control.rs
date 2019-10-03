/*
 * File: src/monitor/control.rs
 * Date: 02.10.2019
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

use std::io;
use std::sync::mpsc::Sender;
use tui::style::Color;
use crossterm::KeyEvent;
use serial_unit_testing::utils::{self, TextFormat};
use super::ui::Monitor;
use super::event::Event;

pub struct Control<'a> {
    ui: Monitor<'a>,

    input: String,
    input_format: TextFormat,
    input_history: Vec<String>,
    input_history_index: i32,
    input_backup: String,

    output: String,
    output_format: TextFormat,

    cursor_position: usize
}

impl<'a> Control<'a> {
    pub fn new(input_format: TextFormat, output_format: TextFormat, io_tx: Sender<(String, TextFormat)>) -> Result<Control<'a>, io::Error> {
        let ui = Monitor::new(io_tx)?;

        Ok(Control {
            ui,
            input: String::new(),
            input_format,
            input_history: vec!(),
            input_history_index: -1,
            input_backup: String::new(),
            output: String::new(),
            output_format,
            cursor_position: 1
        })
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        self.add_control_key(1, "Help");
        self.add_control_key(2, "Input format");
        self.add_control_key(3, "Output format");
        self.add_control_key(4, "Clear");
        self.add_control_key(10, "Close");

        self.ui.run();

        // main loop
        loop {
            self.ui.render(&self.input, &self.output, "Input - /Output -")?;

            match self.ui_rx.recv() {
                Ok(Event::Input(event)) => {
                    // stop execution if true returned
                    if self.handle_keys(event) {
                        break;
                    }
                },
                Ok(Event::CursorTick) => {
                    self.cursor_state = !self.cursor_state;
                },
                Ok(Event::Output(mut data)) => {
                    // filter carriage return characters as they stop newline from working
                    data.retain(|f| *f != 13);

                    let text = utils::radix_string(&data, &self.output_format);

                    self.output.push_str(&text);
                },
                Ok(Event::Error(text)) => {
                    self.error = Some(text);
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn reset_input(&mut self) {
        self.input.clear();

        self.cursor_position = 1;
    }

    fn advance_cursor(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }

    fn retreat_cursor(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    fn add_control_key(&mut self, num: u32, name: &str) {
        self.ui.add_control_text(format!("F{}", num));
        self.ui.add_control_text_with_color(name.to_string(), Color::Cyan);
    }

    fn handle_keys(&mut self, event: KeyEvent) -> bool {
        match event {
            KeyEvent::Char(c) => {
                if c == '\n' {
                    // send io event with text
                    let mut text = self.input.clone();

                    // TODO: Add newline in every format
                    // TODO: Add option to toggle newline append
                    if self.input_format == TextFormat::Text {
                        text.push('\n');
                    }

                    if let Err(_err) = self.io_tx.send((text, self.input_format.clone())) {
                        self.error = Some("Unable to send event to I/O thread".to_string());
                    }

                    // add history entry if input has changed
                    if self.input.is_empty() == false {
                        if let Some(last_input) = self.input_history.first() {
                            if *last_input != self.input {
                                self.input_history.insert(0, self.input.clone());
                            }
                        } else {
                            self.input_history.insert(0, self.input.clone());
                        }
                    }

                    // reset input and history
                    self.input_history_index = -1;
                    self.input_backup.clear();

                    self.reset_input();
                } else {
                    if self.cursor_position < self.input.len() {
                        self.input.insert(self.cursor_position, c);
                    } else {
                        self.input.push(c);
                    }

                    self.advance_cursor();
                }
            },
            KeyEvent::Ctrl(c) => {
                if c == 'c' {
                    // Exit application
                    return true;
                }
            },
            KeyEvent::Backspace => {
                if self.input.is_empty() == false && self.cursor_position != 0 {
                    if self.cursor_position < self.input.len() {
                        self.input.remove(self.cursor_position - 1);
                    } else {
                        self.input.pop();
                    }

                    self.retreat_cursor();
                }
            },
            KeyEvent::Delete => {
                if self.input.is_empty() == false && self.cursor_position < self.input.len() {
                    self.input.remove(self.cursor_position);
                }
            },
            KeyEvent::Left => {
                self.retreat_cursor();
            },
            KeyEvent::Right => {
                self.advance_cursor();
            },
            // TODO: Replace with settings window/shortcuts
            KeyEvent::Up => {
                if self.input_history.is_empty() {
                    return false;
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
                self.cursor_position = self.input.len();
            },
            KeyEvent::Down => {
                if self.input_history_index == -1 {
                    return false;
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

                self.cursor_position = self.input.len();
            },
            KeyEvent::Esc => {
                return true;
            },
            KeyEvent::F(num) => {
                match num {
                    2 => {
                        self.input_format = Monitor::get_next_format(&self.input_format);
                    },
                    3 => {
                        self.output_format = Monitor::get_next_format(&self.output_format);
                    },
                    4 => {
                        self.output.clear();
                    },
                    10 => {
                        return true;
                    },
                    _ => ()
                };
            },
            _ => {}
        }

        return false;
    }

    fn get_input_render_text(&mut self) -> String {
        // do not show input when an error is detected
        if self.error.is_some() {
            return "Press ESC key to close".to_string();
        }

        if self.cursor_state == false {
            return self.input.clone();
        }

        let mut input = self.input.clone();

        // place cursor in input text
        if self.input.is_empty() == false && self.cursor_position < self.input.len() {
            input.remove(self.cursor_position);
        }

        if self.cursor_position < self.input.len() {
            input.insert(self.cursor_position, '█');
        } else {
            input.push('█');
        }

        input
    }

    fn get_format_name(format: &TextFormat) -> &str {
        match format {
            TextFormat::Text => "Text",
            TextFormat::Binary => "Binary",
            TextFormat::Octal => "Octal",
            TextFormat::Decimal => "Decimal",
            TextFormat::Hex => "Hexadecimal"
        }
    }

    fn get_next_format(format: &TextFormat) -> TextFormat {
        match format {
            TextFormat::Text => TextFormat::Binary,
            TextFormat::Binary => TextFormat::Octal,
            TextFormat::Octal => TextFormat::Decimal,
            TextFormat::Decimal => TextFormat::Hex,
            TextFormat::Hex => TextFormat::Text
        }
    }
}
