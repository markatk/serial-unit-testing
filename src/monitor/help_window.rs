/*
 * File: src/monitor/help_window.rs
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

use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Modifier, Color};
use crossterm::KeyEvent;
use super::{Window, WindowManager};

#[derive(Debug, Clone)]
struct HelpEntry {
    pub key: String,
    pub text: String
}

impl HelpEntry {
    pub fn new(key: String, text: String) -> HelpEntry {
        HelpEntry {
            key,
            text
        }
    }

    pub fn get_entry(&self, length: usize) -> [Text; 3] {
        [
            Text::raw(std::iter::repeat(" ").take(length - self.key.len()).collect::<String>()),
            Text::styled(self.key.clone(), Style::default().bg(Color::Cyan)),
            Text::raw(format!(" - {}\n\n", self.text))
        ]
    }
}

pub struct HelpWindow {
    help_entries: Vec<HelpEntry>,
    should_close: bool
}

impl HelpWindow {
    pub fn new() -> HelpWindow {
        HelpWindow {
            help_entries: vec!(),
            should_close: false
        }
    }

    pub fn add_hot_key(&mut self, hot_key: &str, description: &str) {
        self.help_entries.push(HelpEntry::new(hot_key.to_string(), description.to_string()));
    }

    fn get_help_text_entries(help_entries: &Vec<HelpEntry>) -> Vec<Text> {
        // get longest key
        let mut length = 0;

        for entry in help_entries {
            if entry.key.len() > length {
                length = entry.key.len();
            }
        }

        // create text entries
        let title_text = format!("Key{}Action\n\n", std::iter::repeat(" ").take(length).collect::<String>());
        let mut help_text = vec!(Text::styled(title_text, Style::default().modifier(Modifier::BOLD)));

        for entry in help_entries {
            help_text.extend_from_slice(&entry.get_entry(length));
        }

        help_text
    }
}

impl Window for HelpWindow {
    fn run(&mut self, _: &WindowManager) -> Result<(), std::io::Error> {
        self.add_hot_key("F1", "Show help window");
        self.add_hot_key("F2", "Change input format");
        self.add_hot_key("F3", "Change output format");
        self.add_hot_key("F4", "Clear output text");
        self.add_hot_key("F5", "Change appended newline on send");
        self.add_hot_key("F10", "Close application");
        self.add_hot_key("Enter", "Send input to serial");
//        self.add_hot_key("Shift + Enter", "Newline instead of sending input");
        self.add_hot_key("Up", "Go up in input history entries");
        self.add_hot_key("Down", "Go down in input history entries");
        self.add_hot_key("Ctrl + C", "Close application");

        Ok(())
    }

    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend>) -> Result<(), std::io::Error> {
        let help_text = HelpWindow::get_help_text_entries(&self.help_entries);

        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(2)
                ].as_ref())
                .split(f.size());

            let exit_text = [Text::styled("Press ESC to exit help", Style::default().modifier(Modifier::ITALIC))];

            // draw widgets
            Paragraph::new(help_text.iter())
                .block(Block::default()
                    .title("Help")
                    .title_style(Style::default().modifier(Modifier::BOLD))
                    .borders(Borders::ALL))
                .render(&mut f, chunks[0]);

            Paragraph::new(exit_text.iter())
                .block(Block::default())
                .wrap(true)
                .render(&mut f, chunks[1]);
        })
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event {
            KeyEvent::Esc => {
                self.should_close = true;
            },
            _ => {}
        };
    }

    fn should_close(&self) -> bool {
        self.should_close
    }
}