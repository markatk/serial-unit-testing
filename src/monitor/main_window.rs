/*
 * File: src/monitor/main_window.rs
 * Date: 31.10.2019
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
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Modifier, Color};
use crossterm::KeyEvent;
use serial_unit_testing::utils::{self, TextFormat};
use super::enums::NewlineFormat;
use super::helpers;
use crate::windows::{Window, WindowManager, Event};
use super::help_window::HelpWindow;

pub struct MainWindow<'a> {
    window_manager: &'a mut WindowManager<'a>,
    should_close: bool,
    title: String,
    control_text: Vec<Text<'a>>,

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
    cursor_state: bool,

    io_tx: Sender<(String, TextFormat)>,

    help_window: HelpWindow
}

impl<'a> MainWindow<'a> {
    pub fn new(window_manager: &'a mut WindowManager<'a>, input_format: TextFormat, output_format: TextFormat, io_tx: Sender<(String, TextFormat)>, title: String) -> MainWindow<'a> {
        MainWindow {
            window_manager,
            should_close: false,
            title,
            control_text: vec!(),
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
            io_tx,
            help_window: HelpWindow::new()
        }
    }

    pub fn add_control_text(&mut self, text: String) {
        self.control_text.push(Text::raw(text));
    }

    pub fn add_control_text_with_color(&mut self, text: String, color: Color) {
        self.control_text.push(Text::styled(text, Style::default().bg(color)));
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
        self.add_control_text(format!("F{}", num));
        self.add_control_text_with_color(format!("{} ", name), Color::Cyan);
    }

    fn get_last_lines(text: &str, n: usize) -> String {
        text
            .lines()
            .rev()
            .take(n)
            .fold("".to_string(), |current, line| {
                let mut result = line.to_string();
                result.push('\n');
                result.push_str(&current);

                result
            })
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

        if let Err(_err) = self.io_tx.send((text, self.input_format.clone())) {
            self.error = Some("Unable to send event to I/O thread".to_string());

            // TODO: early return?
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

impl<'a> Window for MainWindow<'a> {
    fn setup(&mut self) -> Result<(), io::Error> {
        self.add_control_key(1, "Help");
        self.add_control_key(2, "Input format");
        self.add_control_key(3, "Output format");
        self.add_control_key(4, "Clear");
        self.add_control_key(5, "Newline");
        self.add_control_key(10, "Close");

        Ok(())
    }

    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend>) -> Result<(), io::Error> {
        let input = self.get_input_render_text();
        let control_text = &self.control_text;
        let title = &self.title;
        let error = &self.error;
        let input_title = format!("Input - {}/Output - {}/Newline - {} ",
                                  helpers::get_format_name(&self.input_format),
                                  helpers::get_format_name(&self.output_format),
                                  helpers::get_newline_format_name(&self.newline_format));
        let output = &self.output;

        terminal.draw(|mut f| {
            // create constraints
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(4),
                    Constraint::Length(2)
                ].as_ref())
                .split(f.size());

            // get input and output styled text
            let input_text = if error.is_none() {
                [Text::raw(input)]
            } else {
                [Text::styled(input, Style::default().modifier(Modifier::BOLD))]
            };

            let mut output_text = vec![
                Text::raw(MainWindow::get_last_lines(output, chunks[0].height as usize))
            ];

            if let Some(err) = error {
                output_text.push(Text::styled(format!("\nERROR: {}", err), Style::default().modifier(Modifier::BOLD)))
            }

            // draw widgets into constraints
            Paragraph::new(output_text.iter())
                .block(
                    Block::default()
                        .title(title)
                        .title_style(Style::default().modifier(Modifier::BOLD))
                        .borders(Borders::TOP))
                .wrap(true)
                .render(&mut f, chunks[0]);

            Paragraph::new(input_text.iter())
                .block(
                    Block::default()
                        .title(input_title.as_str())
                        .borders(Borders::TOP))
                .wrap(true)
                .render(&mut f, chunks[1]);

            Paragraph::new(control_text.iter())
                .block(
                    Block::default())
                .wrap(true)
                .render(&mut f, chunks[2]);
        })
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event {
            KeyEvent::Char(c) => {
                if c == '\n' {
                    self.send_input();

                    return;
                }

                // add character to input
                if self.cursor_position < helpers::char_count(&self.input) {
                    helpers::insert_char(&mut self.input, self.cursor_position, c);
                } else {
                    self.input.push(c);
                }

                self.advance_cursor();
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
                self.should_close = true;
            },
            KeyEvent::F(num) => {
                match num {
                    1 => {
//                        window_manager.push_window(&mut self.help_window);
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
                        self.should_close = true;
                    },
                    _ => ()
                };
            },
            _ => {}
        };
    }

    fn handle_tick(&mut self, _tick_rate: u64) {
        self.cursor_state = !self.cursor_state;
    }

    fn handle_event(&mut self, event: Event<KeyEvent>) {
        match event {
            Event::Output(mut data) => {
                // filter carriage return characters as they stop newline from working
                // TODO: Replace with lf if no line feed afterwards
                data.retain(|f| *f != 13);

                let text = utils::radix_string(&data, &self.output_format);

                self.output.push_str(&text);
            },
            Event::Error(text) => {
                self.error = Some(text);
            },
            _ => {}
        }
    }

    fn should_close(&self) -> bool {
        self.should_close
    }
}