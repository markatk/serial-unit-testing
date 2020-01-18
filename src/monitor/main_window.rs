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
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
use serial_unit_testing::utils::{self, TextFormat};
use super::help_window::HelpWindow;
use super::text_storage::TextStorage;
use crate::windows::{Window, Event, EventResult, WindowError};

pub struct MainWindow<'a> {
    should_close: bool,
    pub title: String,
    control_text: Vec<Text<'a>>,
    pub text_storage: TextStorage,
    error: Option<WindowError>,
    io_tx: Sender<(String, TextFormat)>,
    cursor_state: bool
}

impl<'a> MainWindow<'a> {
    pub fn new(io_tx: Sender<(String, TextFormat)>) -> Box<MainWindow<'a>> {
        let mut control_text = vec!();

        MainWindow::add_control_key(&mut control_text, 1, "Help");
        MainWindow::add_control_key(&mut control_text, 2, "Input format");
        MainWindow::add_control_key(&mut control_text, 3, "Output format");
        MainWindow::add_control_key(&mut control_text, 4, "Clear");
        MainWindow::add_control_key(&mut control_text, 5, "Newline");
        MainWindow::add_control_key(&mut control_text, 6, "Input escape");
        MainWindow::add_control_key(&mut control_text, 10, "Close");

        Box::new(MainWindow {
            should_close: false,
            title: String::new(),
            control_text,
            text_storage: Default::default(),
            error: None,
            io_tx,
            cursor_state: false
        })
    }

    fn add_control_text(entries: &mut Vec<Text<'a>>, text: String) {
        entries.push(Text::raw(text));
    }

    fn add_control_text_with_color(entries: &mut Vec<Text<'a>>, text: String, color: Color) {
        entries.push(Text::styled(text, Style::default().bg(color)));
    }

    fn add_control_key(entries: &mut Vec<Text<'a>>, num: u8, name: &str) {
        MainWindow::add_control_text(entries, format!("F{}", num));
        MainWindow::add_control_text_with_color(entries, format!("{} ", name), Color::Cyan);
    }

    fn get_input_render_text(&mut self) -> String {
        // do not show input when an error is detected
        if let Some(error) = &self.error {
            let close_message = if error.recoverable {
                "Press ESC to continue"
            } else {
                "Press ESC to close"
            };

            return format!("Error: {}. {}", error.description, close_message);
        }

        if self.cursor_state == false {
            return self.text_storage.get_input();
        }

        let mut input = self.text_storage.get_input();
        let cursor_position = self.text_storage.get_cursor_position();

        // place cursor in input text
        if input.is_empty() == false && cursor_position < utils::char_count(&input) {
            utils::remove_char(&mut input, cursor_position);
        }

        if cursor_position < utils::char_count(&input) {
            utils::insert_char(&mut input, cursor_position, '█');
        } else {
            input.push('█');
        }

        input
    }

    fn send_input(&mut self) {
        // send io event with text
        let mut text = self.text_storage.get_input();

        utils::add_newline(&mut text, self.text_storage.input_format, self.text_storage.newline_format);

        if self.text_storage.escape_input {
            text = utils::escape_text(text);
        }

        if let Err(_err) = self.io_tx.send((text, self.text_storage.input_format)) {
            self.set_error("Unable to send event to I/O thread".to_string(), false);

            // TODO: early return?
        }

        // add history entry if input has changed
        if self.text_storage.is_input_empty() == false {
            self.text_storage.add_history_entry();
        }
        self.text_storage.reset_input();
    }

    fn set_error(&mut self, message: String, recoverable: bool) {
        self.error = Some(WindowError::new(message, recoverable));
    }

    fn get_bool(value: bool) -> &'static str {
        match value {
            true => "Yes",
            false => "No"
        }
    }
}

impl<'a> Window for MainWindow<'a> {
    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<(), io::Error> {
        let input = self.get_input_render_text();
        let control_text = &self.control_text;
        let title = &self.title;
        let error = &self.error;
        let input_title = format!("Input - {}/Output - {}/Newline - {}/Escape input - {} ",
                                  utils::get_format_name(&self.text_storage.input_format),
                                  utils::get_format_name(&self.text_storage.output_format),
                                  utils::get_newline_format_name(&self.text_storage.newline_format),
                                  MainWindow::get_bool(self.text_storage.escape_input));
        let text_storage = &mut self.text_storage;

        terminal.draw(|mut f| {
            // create constraints
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(1),
                    Constraint::Length(4),
                    Constraint::Length(1)
                ].as_ref())
                .split(f.size());

            // get input and output styled text
            let input_text = if error.is_none() {
                [Text::raw(input)]
            } else {
                [Text::styled(input, Style::default().modifier(Modifier::BOLD).bg(Color::Red))]
            };

            let (output, line_counter) = text_storage.get_output_lines(chunks[0].height as usize - 1);
            let line_spaces = chunks[1].width as usize - line_counter.len() - 1;

            let output_text = vec![
                Text::raw(output)
            ];

            let line_text = vec![
                Text::raw(format!("{}{}", std::iter::repeat(" ").take(line_spaces).collect::<String>(), line_counter))
            ];

            // draw widgets into constraints
            Paragraph::new(output_text.iter())
                .block(
                    Block::default()
                        .title(title)
                        .title_style(Style::default().modifier(Modifier::BOLD))
                        .borders(Borders::TOP))
                .wrap(true)
                .render(&mut f, chunks[0]);

            Paragraph::new(line_text.iter())
                .render(&mut f, chunks[1]);

            Paragraph::new(input_text.iter())
                .block(
                    Block::default()
                        .title(input_title.as_str())
                        .borders(Borders::TOP))
                .wrap(true)
                .render(&mut f, chunks[2]);

            Paragraph::new(control_text.iter())
                .render(&mut f, chunks[3]);
        })
    }

    fn handle_key_event<'b>(&mut self, event: KeyEvent) -> EventResult {
        let mut result = EventResult::new();

        match event {
            KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::CONTROL } => {
                match c {
                    'a' => self.text_storage.cursor_at_beginning(),
                    'd' => self.text_storage.remove_character(false),
                    'e' => self.text_storage.cursor_at_end(),
                    'h' => self.text_storage.remove_character(true),
                    'l' => self.text_storage.reset_output(),
                    _ => ()
                }
            },
            KeyEvent { code: KeyCode::Char(c), modifiers: _ } => {
                if c == '\n' {
                    self.send_input();
                } else {
                    self.text_storage.input_add(c);
                }
            },
            KeyEvent { code: KeyCode::Backspace, modifiers: _ } => self.text_storage.remove_character(true),
            KeyEvent { code: KeyCode::Delete, modifiers: _ } => self.text_storage.remove_character(false),
            KeyEvent { code: KeyCode::Left, modifiers: _ } => self.text_storage.retreat_cursor(),
            KeyEvent { code: KeyCode::Right, modifiers: _ } => self.text_storage.advance_cursor(),
            KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::SHIFT } => self.text_storage.retreat_output(),
            KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::SHIFT } => self.text_storage.advance_output(),
            KeyEvent { code: KeyCode::Up, modifiers: _ } => self.text_storage.advance_history(),
            KeyEvent { code: KeyCode::Down, modifiers: _ } => self.text_storage.retreat_history(),
            KeyEvent { code: KeyCode::Esc, modifiers: _ } => {
                if let Some(ref err) = self.error {
                    if err.recoverable {
                        self.error = None;
                        self.text_storage.advance_history();

                        return result;
                    }
                }

                self.should_close = true
            },
            KeyEvent { code: KeyCode::Home, modifiers: _ } => self.text_storage.cursor_at_beginning(),
            KeyEvent { code: KeyCode::End, modifiers: _ } => self.text_storage.cursor_at_end(),
            KeyEvent { code: KeyCode::F(num), modifiers: _ } => {
                match num {
                    1 => result.child = Some(HelpWindow::new()),
                    2 => self.text_storage.input_format = utils::get_next_format(&self.text_storage.input_format),
                    3 => self.text_storage.output_format = utils::get_next_format(&self.text_storage.output_format),
                    4 => self.text_storage.reset_output(),
                    5 => self.text_storage.newline_format = utils::get_next_newline_format(&self.text_storage.newline_format),
                    6 => self.text_storage.escape_input = !self.text_storage.escape_input,
                    10 => self.should_close = true,
                    _ => ()
                };
            },
            _ => {}
        };

        result
    }

    fn handle_tick(&mut self, _tick_rate: u64) -> EventResult {
        self.cursor_state = !self.cursor_state;

        EventResult::new()
    }

    fn handle_event(&mut self, event: Event<KeyEvent>) -> EventResult {
        match event {
            Event::Output(mut data) => {
                // filter carriage return characters as they stop newline from working
                // TODO: Replace with lf if no line feed afterwards
                data.retain(|f| *f != 13);

                // TODO: Handle error
                let text = utils::radix_string(&data, &self.text_storage.output_format).unwrap();

                self.text_storage.output_add_str(&text);
            },
            Event::Error(error) => {
                self.set_error(error.description, error.recoverable);
            },
            _ => {}
        };

        EventResult::new()
    }

    fn should_close(&self) -> bool {
        self.should_close
    }
}