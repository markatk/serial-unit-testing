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
use crossterm::event::{KeyEvent, KeyCode};
use crate::windows::{Window, EventResult};

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
    should_close: bool,
    page: u32,
    page_count: u32
}

impl HelpWindow {
    pub fn new() -> Box<HelpWindow> {
        let mut help_entries= vec!();

        HelpWindow::add_hot_key(&mut help_entries, "F1", "Show help window");
        HelpWindow::add_hot_key(&mut help_entries, "F2", "Change the input format");
        HelpWindow::add_hot_key(&mut help_entries, "F3", "Change the output format");
        HelpWindow::add_hot_key(&mut help_entries, "F4", "Clear the output text");
        HelpWindow::add_hot_key(&mut help_entries, "F5", "Change appended newline on send");
        HelpWindow::add_hot_key(&mut help_entries, "F10", "Close the application");
        HelpWindow::add_hot_key(&mut help_entries, "Enter", "Send the input to serial");
//        HelpWindow::add_hot_key(&mut help_entries, "Shift + Enter", "Newline instead of sending input");
        HelpWindow::add_hot_key(&mut help_entries, "Up", "Go up in input history entries");
        HelpWindow::add_hot_key(&mut help_entries, "Down", "Go down in input history entries");
        HelpWindow::add_hot_key(&mut help_entries, "Ctrl + C", "Close application");
        HelpWindow::add_hot_key(&mut help_entries, "Ctrl + A", "Goto the beginning of the input (same as home)");
        HelpWindow::add_hot_key(&mut help_entries, "Ctrl + E", "Goto the end of the input (same as end)");
        HelpWindow::add_hot_key(&mut help_entries, "Ctrl + D", "Delete the character under the cursor (same as delete)");
        HelpWindow::add_hot_key(&mut help_entries, "Ctrl + H", "Delete the character in front of the cursor (same as backspace)");
        HelpWindow::add_hot_key(&mut help_entries, "Ctrl + L", "Clear the output text");

        Box::new(HelpWindow {
            help_entries,
            should_close: false,
            page: 0,
            page_count: 1
        })
    }

    fn get_help_text_entries(help_entries: &Vec<HelpEntry>, skip: usize, count: usize) -> Vec<Text> {
        let entries = help_entries
            .iter()
            .skip(skip)
            .take(count)
            .collect::<Vec<_>>();

        // get longest key
        let mut length = 0;

        for entry in entries.iter() {
            if entry.key.len() > length {
                length = entry.key.len();
            }
        }

        if length < 3 {
            length = 3;
        }

        // create text entries
        let title_text = format!("{}Key   Action\n\n", std::iter::repeat(" ").take(length - 3).collect::<String>());
        let mut help_text = vec!(Text::styled(title_text, Style::default().modifier(Modifier::BOLD)));

        for entry in entries {
            help_text.extend_from_slice(&entry.get_entry(length));
        }

        help_text
    }

    fn add_hot_key(entries: &mut Vec<HelpEntry>, hot_key: &str, description: &str) {
        entries.push(HelpEntry::new(hot_key.to_string(), description.to_string()));
    }
}

impl Window for HelpWindow {
    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<(), std::io::Error> {
        let help_entries = &self.help_entries;
        let page_count = &mut self.page_count;
        let page = &mut self.page;

        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(2)
                ].as_ref())
                .split(f.size());

            // calculate page metrics
            let entries_per_page = (chunks[0].height - 3) / 2;

            *page_count = help_entries.len() as u32 / entries_per_page as u32 + 1;
            if *page > *page_count {
                *page = *page_count - 1;
            }

            let help_text = HelpWindow::get_help_text_entries(help_entries, entries_per_page as usize * *page as usize, entries_per_page as usize);

            let exit_str = if *page_count > 1 {
                "Press left or right to change pages, ESC to exit help"
            } else {
                "Press ESC to exit help"
            };

            let exit_text = [Text::styled(exit_str, Style::default().modifier(Modifier::ITALIC))];

            // draw widgets
            Paragraph::new(help_text.iter())
                .block(Block::default()
                    .title(format!("Help: {}/{}", *page + 1, page_count).as_str())
                    .title_style(Style::default().modifier(Modifier::BOLD))
                    .borders(Borders::ALL))
                .render(&mut f, chunks[0]);

            Paragraph::new(exit_text.iter())
                .block(Block::default())
                .wrap(true)
                .render(&mut f, chunks[1]);
        })
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> EventResult {
        match event {
            KeyEvent { code: KeyCode::Esc, modifiers: _ } => {
                self.should_close = true;
            },
            KeyEvent { code: KeyCode::Left, modifiers: _ }  => {
                if self.page > 0 {
                    self.page -= 1;
                }
            },
            KeyEvent { code: KeyCode::Right, modifiers: _ } => {
                if self.page < self.page_count - 1 {
                    self.page += 1;
                }
            },
            _ => {}
        };

        EventResult::new()
    }

    fn should_close(&self) -> bool {
        self.should_close
    }
}