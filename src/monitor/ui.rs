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
use crossterm::{input, InputEvent, KeyEvent, AlternateScreen};
use super::event::Event;

pub struct Monitor {
    terminal: Terminal<CrosstermBackend>,

    input: String,
    output: String,
    cursor_state: bool,
    cursor_position: usize,

    pub ui_tx: Sender<Event<KeyEvent>>,
    ui_rx: Receiver<Event<KeyEvent>>,
    io_tx: Sender<String>

    // TODO: Add input text type
    // TODO: Add output text type
    // TODO: Add input shortcuts
    // TODO: Add output shortcuts
    // TODO: Add output scrolling
    // TODO: Add multiline input
}

impl Monitor {
    pub fn new(io_tx: Sender<String>) -> Result<Monitor, io::Error> {
        let screen = AlternateScreen::to_alternate(true)?;
        let backend = CrosstermBackend::with_alternate_screen(screen)?;
        let terminal = Terminal::new(backend)?;

        let (ui_tx, ui_rx) = mpsc::channel();

        Ok(Monitor {
            terminal,
            input: String::new(),
            output: String::new(),
            cursor_state: false,
            cursor_position: 1,
            ui_tx,
            ui_rx,
            io_tx
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
        self.terminal.clear()?;

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
                Ok(Event::Output(text)) => {
                    self.output.push_str(&text);
                },
                Ok(Event::Error(text)) => {
                    // TODO: Handle proper disconnect message
                    return Err(io::Error::new(io::ErrorKind::Other, text));
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn render(&mut self) -> Result<(), io::Error> {
        let mut input = self.input.clone();
        let output = &self.output;
        let cursor_state = self.cursor_state;

        // place cursor in input text
        if cursor_state {
            if self.input.is_empty() == false && self.cursor_position < self.input.len() {
                input.remove(self.cursor_position);
            }

            if self.cursor_position < self.input.len() {
                input.insert(self.cursor_position, '█');
            } else {
                input.push('█');
            }
        }

        self.terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(4)
                ].as_ref())
                .split(f.size());

            let input_text = [
                Text::raw(input)
            ];

            let output_text = [
                Text::raw(output)
            ];

            Paragraph::new(output_text.iter())
                .block(
                    Block::default())
                .wrap(true)
                .render(&mut f, chunks[0]);

            Paragraph::new(input_text.iter())
                .block(
                    Block::default()
                        .title("Input - Text")
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
                    text.push('\n');

                    if let Err(_err) = self.io_tx.send(text) {
                        // TODO: Handle error
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
            KeyEvent::Esc => {
                return true;
            },
            _ => {}
        }

        return false;
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
}
