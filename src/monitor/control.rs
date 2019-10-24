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
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{Sender, Receiver};
use tui::style::Color;
use crossterm::KeyEvent;
use serial_unit_testing::utils::{self, TextFormat};
use super::ui::Monitor;
use super::enums::{Event, NewlineFormat};
use super::helpers;

pub struct Control<'a> {
    ui: Monitor<'a>,

    input: String,
    input_format: TextFormat,
    input_history: Vec<String>,
    input_history_index: i32,
    input_backup: String,
    newline_format: NewlineFormat,

    output: String,
    output_format: TextFormat,

    error: Option<String>,

    cursor_position: usize,
    // TODO: move cursor state into monitor struct
    cursor_state: bool,
    show_help: bool
}

impl<'a> Control<'a> {
    pub fn new(input_format: TextFormat, output_format: TextFormat, io_tx: Sender<(String, TextFormat)>, title: String) -> Result<Control<'a>, io::Error> {
        let ui = Monitor::new(io_tx, title)?;

        Ok(Control {
            ui,
            input: String::new(),
            input_format,
            input_history: vec!(),
            input_history_index: -1,
            input_backup: String::new(),
            newline_format: NewlineFormat::LineFeed,
            output: String::new(),
            output_format,
            error: None,
            cursor_position: 1,
            cursor_state: false,
            show_help: false
        })
    }

    pub fn get_ui_tx(&self) -> &Sender<Event<KeyEvent>> {
        &self.ui.ui_tx
    }

    pub fn get_ui_rx(&self) -> &Receiver<Event<KeyEvent>> {
        &self.ui.ui_rx
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        self.add_control_key(1, "Help");
        self.add_control_key(2, "Input format");
        self.add_control_key(3, "Output format");
        self.add_control_key(4, "Clear");
        self.add_control_key(5, "Newline");
        self.add_control_key(10, "Close");

        self.ui.run()?;

        // TODO: Update in render call
        {
            let tx = self.get_ui_tx().clone();
            thread::spawn(move || {
                loop {
                    tx.send(Event::CursorTick).unwrap();

                    thread::sleep(Duration::from_millis(500));
                }
            });
        }

        // main loop
        loop {
            if self.show_help {
                self.ui.render_help()?;
            } else {
                let input = self.get_input_render_text();
                let input_title = format!("Input - {}/Output - {}/Newline - {} ",
                                          helpers::get_format_name(&self.input_format),
                                          helpers::get_format_name(&self.output_format),
                                          helpers::get_newline_format_name(&self.newline_format));

                self.ui.render(input.as_str(), &self.output, input_title.as_str(), &self.error)?;
            }

            match self.get_ui_rx().recv() {
                Ok(Event::Input(event)) => {
                    // stop execution if true returned
                    // TODO: replace return with flag in struct
                    if self.handle_keys(event) {
                        break;
                    }
                },
                Ok(Event::CursorTick) => {
                    self.cursor_state = !self.cursor_state;
                },
                Ok(Event::Output(mut data)) => {
                    // filter carriage return characters as they stop newline from working
                    // TODO: Replace with lf if no line feed afterwards
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
        if self.cursor_position < helpers::char_count(&self.input) {
            self.cursor_position += 1;
        }
    }

    fn retreat_cursor(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    fn add_control_key(&mut self, num: u8, name: &str) {
        self.ui.add_control_text(format!("F{}", num));
        self.ui.add_control_text_with_color(format!("{} ", name), Color::Cyan);
    }

    fn handle_keys(&mut self, event: KeyEvent) -> bool {
        match event {
            KeyEvent::Char(c) => {
                if c == '\n' {
                    self.send_input();

                    return false;
                }

                // add character to input
                if self.cursor_position < helpers::char_count(&self.input) {
                    helpers::insert_char(&mut self.input, self.cursor_position, c);
                } else {
                    self.input.push(c);
                }

                self.advance_cursor();
            },
            KeyEvent::Ctrl(c) => {
                if c == 'c' {
                    // Exit application
                    return true;
                }
            },
            KeyEvent::Backspace => {
                if self.input.is_empty() == false && self.cursor_position != 0 {
                    if self.cursor_position < helpers::char_count(&self.input) {
                        helpers::remove_char(&mut self.input, self.cursor_position - 1);
                    } else {
                        self.input.pop();
                    }

                    self.retreat_cursor();
                }
            },
            KeyEvent::Delete => {
                if self.input.is_empty() == false && self.cursor_position < helpers::char_count(&self.input) {
                    helpers::remove_char(&mut self.input, self.cursor_position);
                }
            },
            KeyEvent::Left => {
                self.retreat_cursor();
            },
            KeyEvent::Right => {
                self.advance_cursor();
            },
            KeyEvent::Up => {
                self.advance_history();
            },
            KeyEvent::Down => {
                self.retreat_history();
            },
            KeyEvent::Esc => {
                if self.show_help {
                    self.show_help = false;
                } else {
                    // Exit application
                    return true;
                }
            },
            KeyEvent::F(num) => {
                match num {
                    1 => {
                        self.show_help = true;
                    },
                    2 => {
                        self.input_format = helpers::get_next_format(&self.input_format);
                    },
                    3 => {
                        self.output_format = helpers::get_next_format(&self.output_format);
                    },
                    4 => {
                        self.output.clear();
                    },
                    5 => {
                        self.newline_format = helpers::get_next_newline_format(&self.newline_format);
                    },
                    10 => {
                        // Exit application
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
        // TODO: move method into monitor struct
        // do not show input when an error is detected
        if self.error.is_some() {
            return "Press ESC key to close".to_string();
        }

        if self.cursor_state == false {
            return self.input.clone();
        }

        let mut input = self.input.clone();

        // place cursor in input text
        if self.input.is_empty() == false && self.cursor_position < helpers::char_count(&input) {
            helpers::remove_char(&mut input, self.cursor_position);
        }

        if self.cursor_position < helpers::char_count(&input) {
            helpers::insert_char(&mut input, self.cursor_position, '█');
        } else {
            input.push('█');
        }

        input
    }

    fn send_input(&mut self) {
        // send io event with text
        let mut text = self.input.clone();

        helpers::add_newline(&mut text, self.input_format, self.newline_format);

        if let Err(_err) = self.ui.io_tx.send((text, self.input_format.clone())) {
            self.error = Some("Unable to send event to I/O thread".to_string());

            // early return?
        }

        // add history entry if input has changed
        if self.input.is_empty() == false {
            self.add_history_entry(self.input.clone());
        }

        // reset input and history
        self.input_history_index = -1;
        self.input_backup.clear();

        self.reset_input();
    }

    fn add_history_entry(&mut self, text: String) {
        if let Some(last_input) = self.input_history.first() {
            if *last_input != self.input {
                self.input_history.insert(0, text);
            }
        } else {
            self.input_history.insert(0, text);
        }
    }

    fn advance_history(&mut self) {
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
        self.cursor_position = helpers::char_count(&self.input);
    }

    fn retreat_history(&mut self) {
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

        self.cursor_position = helpers::char_count(&self.input);
    }
}
