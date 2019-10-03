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
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Modifier, Color};
use crossterm::{input, InputEvent, KeyEvent, AlternateScreen};
use super::event::Event;
use serial_unit_testing::utils::TextFormat;

pub struct Monitor<'a> {
    terminal: Terminal<CrosstermBackend>,

    control_text: Vec<Text<'a>>,

    pub ui_tx: Sender<Event<KeyEvent>>,
    pub ui_rx: Receiver<Event<KeyEvent>>,
    pub io_tx: Sender<(String, TextFormat)>,

    // TODO: Add output manual scrolling and current line display in corner
    // TODO: Add multiline input -> shift-enter for sending or shift-enter for multi line?
    // TODO: Support unicode?
    // TODO: Add input escaping
    // TODO: Add help window on F1 with keyboard shortcuts
}

impl<'a> Monitor<'a> {
    pub fn new(io_tx: Sender<(String, TextFormat)>) -> Result<Monitor<'a>, io::Error> {
        let screen = AlternateScreen::to_alternate(true)?;
        let backend = CrosstermBackend::with_alternate_screen(screen)?;

        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        let (ui_tx, ui_rx) = mpsc::channel();

        Ok(Monitor {
            terminal,
            control_text: vec!(),
            ui_tx,
            ui_rx,
            io_tx
        })
    }

    pub fn add_control_text(&mut self, text: String) {
        self.control_text.push(Text::raw(text));
    }

    pub fn add_control_text_with_color(&mut self, text: String, color: Color) {
        self.control_text.push(Text::styled(text, Style::default().bg(color)));
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

        Ok(())
    }

    pub fn render(&mut self, input: &str, output: &str, input_title: &str, error: &Option<String>) -> Result<(), io::Error> {
        let control_text = &self.control_text;

        self.terminal.draw(|mut f| {
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
                        .title(input_title)
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
