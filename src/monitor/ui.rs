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
use std::sync::mpsc;
use std::thread;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction};
use crossterm::{input, InputEvent, KeyEvent, AlternateScreen, Crossterm};

enum Event<I> {
    Input(I),
    CursorTick
}

pub fn show_terminal() -> Result<(), io::Error> {
    let screen = AlternateScreen::to_alternate(true)?;
    let backend = CrosstermBackend::with_alternate_screen(screen)?;
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // register inputs
    let (tx, rx) = mpsc::channel();
    {
        let tx = tx.clone();
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
        let tx = tx.clone();
        thread::spawn(move || {
            loop {
                tx.send(Event::CursorTick).unwrap();

                thread::sleep_ms(500);
            }
        });
    }

    // main loop
    terminal.clear()?;

    let mut input = String::new();
    let mut output = String::new();
    let mut cursor_state = false;

    loop {
        render(&mut terminal, input.as_str(), output.as_str(), cursor_state)?;

        match rx.recv() {
            Ok(Event::Input(event)) => {
                match event {
                    KeyEvent::Char(c) => {
                        if c == '\n' {
                            output.push_str(&input);
                            output.push('\n');

                            input.clear();
                        } else {
                            input.push(c);
                        }
                    },
                    KeyEvent::Ctrl(c) => {
                        if c == 'c' {
                            break;
                        }
                    },
                    KeyEvent::Backspace => {
                        if input.is_empty() == false {
                            input.pop();
                        }
                    },
                    KeyEvent::Esc => {
                        break;
                    },
                    _ => {}
                }
            },
            Ok(Event::CursorTick) => {
                cursor_state = !cursor_state;
            },
            _ => {}
        }
    }

    Ok(())
}

fn render(terminal: &mut Terminal<CrosstermBackend>, input: &str, output: &str, cursor_state: bool) -> Result<(), io::Error> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3)
            ].as_ref())
            .split(f.size());

        let mut input_text = vec![
            Text::raw(input)
        ];

        if cursor_state {
            input_text.push(Text::raw("█"));
        }

        let output_text = [
            Text::raw(output)
        ];

        Paragraph::new(output_text.iter())
            .block(
                Block::default()
            )
            .wrap(true)
            .render(&mut f, chunks[0]);

        Paragraph::new(input_text.iter())
            .block(
                Block::default()
                    .title("Input")
                    .borders(Borders::TOP))
            .wrap(true)
            .render(&mut f, chunks[1]);
    })
}
