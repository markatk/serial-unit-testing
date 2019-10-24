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
use super::event::Event;

#[derive(Debug, Copy, Clone, PartialEq)]
enum NewlineFormat {
    None,
    CarriageReturn,
    LineFeed,
    Both
}

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
            newline_format: NewlineFormat::LineFeed,
            output: String::new(),
            output_format,
            error: None,
            cursor_position: 1,
            cursor_state: false
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
            let input = self.get_input_render_text();
            let input_title = format!("Input - {}/Output - {}/Newline - {} ",
                Control::get_format_name(&self.input_format),
                Control::get_format_name(&self.output_format),
                Control::get_newline_format_name(&self.newline_format));

            self.ui.render(input.as_str(), &self.output, input_title.as_str(), &self.error)?;

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

    fn add_control_key(&mut self, num: u8, name: &str) {
        self.ui.add_control_text(format!("F{}", num));
        self.ui.add_control_text_with_color(format!("{} ", name), Color::Cyan);
    }

    fn handle_keys(&mut self, event: KeyEvent) -> bool {
        match event {
            KeyEvent::Char(c) => {
                // TODO: Use enter key event?
                if c == '\n' {
                    self.send_input();
                } else {
                    // add character to input
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
            KeyEvent::Up => {
                self.advance_history();
            },
            KeyEvent::Down => {
                self.retreat_history();
            },
            KeyEvent::Esc => {
                return true;
            },
            KeyEvent::F(num) => {
                match num {
                    2 => {
                        self.input_format = Control::get_next_format(&self.input_format);
                    },
                    3 => {
                        self.output_format = Control::get_next_format(&self.output_format);
                    },
                    4 => {
                        self.output.clear();
                    },
                    5 => {
                        self.newline_format = Control::get_next_newline_format(&self.newline_format);
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

    fn send_input(&mut self) {
        // send io event with text
        let mut text = self.input.clone();

        Control::add_newline(&mut text, self.input_format, self.newline_format);

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
        self.cursor_position = self.input.len();
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

        self.cursor_position = self.input.len();
    }

    fn add_newline(text: &mut String, text_format: TextFormat, newline_format: NewlineFormat) {
        if newline_format == NewlineFormat::None {
            return;
        }

        let cr = match text_format {
            TextFormat::Text => "\r",
            TextFormat::Binary => "00001101",
            TextFormat::Octal => "015",
            TextFormat::Decimal => "13",
            TextFormat::Hex => "0D"
        };

        let lf = match text_format {
            TextFormat::Text => "\n",
            TextFormat::Binary => "00001010",
            TextFormat::Octal => "012",
            TextFormat::Decimal => "10",
            TextFormat::Hex => "0A"
        };

        if newline_format == NewlineFormat::CarriageReturn || newline_format == NewlineFormat::Both {
            text.push_str(cr);
        }

        if newline_format == NewlineFormat::LineFeed || newline_format == NewlineFormat::Both {
            text.push_str(lf);
        }
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

    fn get_newline_format_name(format: &NewlineFormat) -> &str {
        match format {
            NewlineFormat::None => "None",
            NewlineFormat::CarriageReturn => "Carriage return",
            NewlineFormat::LineFeed => "Line feed",
            NewlineFormat::Both => "Both"
        }
    }

    fn get_next_newline_format(format: &NewlineFormat) -> NewlineFormat {
        match format {
            NewlineFormat::None => NewlineFormat::CarriageReturn,
            NewlineFormat::CarriageReturn => NewlineFormat::LineFeed,
            NewlineFormat::LineFeed => NewlineFormat::Both,
            NewlineFormat::Both => NewlineFormat::None
        }
    }
}
