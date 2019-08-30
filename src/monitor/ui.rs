/*
 * File: src/monitor/ui.rs
 * Date: 20.08.2019
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
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::time::Duration;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Modifier};
use crossterm::{input, InputEvent, KeyEvent, AlternateScreen};
use super::event::Event;
use serial_unit_testing::utils::{self, TextFormat};

pub struct Monitor {
    terminal: Terminal<CrosstermBackend>,

    input: String,
    pub input_format: TextFormat,
    output: String,
    pub output_format: TextFormat,
    cursor_state: bool,
    cursor_position: usize,

    pub ui_tx: Sender<Event<KeyEvent>>,
    ui_rx: Receiver<Event<KeyEvent>>,
    io_tx: Sender<(String, TextFormat)>,

    error: Option<String>

    // TODO: Add input shortcuts -> Change format, History with up and down
    // TODO: Add output shortcuts -> clear output, change format
    // TODO: Add output scrolling
    // TODO: Add multiline input -> shift-enter for sending
    // TODO: Support unicode?
    // TODO: Add input escaping
}

impl Monitor {
    pub fn new(io_tx: Sender<(String, TextFormat)>) -> Result<Monitor, io::Error> {
        let screen = AlternateScreen::to_alternate(true)?;
        let backend = CrosstermBackend::with_alternate_screen(screen)?;
        let terminal = Terminal::new(backend)?;

        let (ui_tx, ui_rx) = mpsc::channel();

        Ok(Monitor {
            terminal,
            input: String::new(),
            input_format: TextFormat::Text,
            output: String::new(),
            output_format: TextFormat::Text,
            cursor_state: false,
            cursor_position: 1,
            ui_tx,
            ui_rx,
            io_tx,
            error: None
        })
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        // start event threads
        {
            let tx = self.ui_tx.clone();
            thread::spawn(move || {
                let input = input();
                let reader = input.read_sync();

                for event in reader {
                    match event {
                        InputEvent::Keyboard(key) => {
                            if let Err(_) = tx.send(Event::Input(key.clone())) {
                                return;
                            }
                        },
                        _ => {}
                    }
                }
            });
        }
        {
            let tx = self.ui_tx.clone();
            thread::spawn(move || {
                loop {
                    tx.send(Event::CursorTick).unwrap();

                    thread::sleep(Duration::from_millis(500));
                }
            });
        }

        // main loop
        self.terminal.hide_cursor()?;

        loop {
            self.render()?;

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

    fn render(&mut self) -> Result<(), io::Error> {
        let input = self.get_input_render_text();
        let input_format = &self.input_format;
        let output = &self.output;
        let output_format = &self.output_format;
        let error = &self.error;

        self.terminal.draw(|mut f| {
            // create constraints
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(4)
                ].as_ref())
                .split(f.size());

            // get input and output styled text
            let input_text = if error.is_none() {
                [Text::raw(input)]
            } else {
                [Text::styled(input, Style::default().modifier(Modifier::BOLD))]
            };

            let mut output_text = vec![
                Text::raw(Monitor::get_last_lines(output, chunks[0].height as usize))
            ];

            if let Some(err) = error {
                output_text.push(Text::styled(format!("\nERROR: {}", err), Style::default().modifier(Modifier::BOLD)))
            }

            // draw widgets into constraints
            Paragraph::new(output_text.iter())
                .block(
                    Block::default())
                .wrap(true)
                .render(&mut f, chunks[0]);

            Paragraph::new(input_text.iter())
                .block(
                    Block::default()
                        .title(format!("Input - {} / Output - {}",
                                       Monitor::get_format_name(input_format),
                                       Monitor::get_format_name(output_format)).as_str())
                        .borders(Borders::TOP))
                .wrap(true)
                .render(&mut f, chunks[1]);
        })
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

                    self.input.clear();

                    self.reset_cursor();
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
                self.input_format = Monitor::get_next_format(&self.input_format);
            },
            KeyEvent::Down => {
                self.output_format = Monitor::get_next_format(&self.output_format);
            },
            KeyEvent::Esc => {
                return true;
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

    fn reset_cursor(&mut self) {
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
}
