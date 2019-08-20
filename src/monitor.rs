/*
 * File: src/monitor.rs
 * Date: 13.09.2018
 * Author: MarkAtk
 * 
 * MIT License
 * 
 * Copyright (c) 2018 MarkAtk
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

use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use clap::{ArgMatches, App, SubCommand};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph, Text};
use tui::layout::{Layout, Constraint, Direction};
use crossterm::{input, InputEvent, KeyEvent, AlternateScreen};
use serial_unit_testing::utils;
use serial_unit_testing::serial::Serial;
use crate::commands;
use std::process::exit;

enum Event<I> {
    Input(I),
    Tick
}

pub fn run(_matches: &ArgMatches) -> Result<(), String> {
//    let (settings, port_name) = commands::get_serial_settings(matches).unwrap();
//
//    match Serial::open_with_settings(port_name, &settings) {
//        Ok(mut serial) => {
//            let text_format = commands::get_text_output_format(matches);
//
//            read(&mut serial, &text_format)
//        },
//        Err(e) => return Err(format!("Error opening port {:?}", e))
//    }

    match show_terminal() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("monitor")
        .about("Continuously display serial port data")
        .args(commands::serial_arguments(false, true).as_slice())
}

fn show_terminal() -> Result<(), io::Error> {
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

    // main loop
    terminal.clear()?;
    let mut input = String::new();

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(3)
                ].as_ref())
                .split(f.size());

            let text = [
                Text::raw(&input)
            ];

            Paragraph::new(text.iter())
                .block(
                    Block::default()
                        .title("Input")
                        .borders(Borders::TOP))
                .wrap(true)
                .render(&mut f, chunks[1]);
        })?;

        if let Ok(Event::Input(event)) = rx.recv() {
            match event {
                KeyEvent::Char(c) => {
                    if c == '\n' {
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
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn read(serial: &mut Serial, text_format: &utils::TextFormat) -> Result<(), String> {
    let mut row_entries = 0;

    loop {
        match serial.read() {
            Ok(bytes) => {
                match text_format {
                    utils::TextFormat::Text => io::stdout().write_all(bytes).unwrap(),
                    _ => utils::print_radix_string(bytes, &text_format, &mut row_entries)
                };

                io::stdout().flush().unwrap();
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => return Err(format!("{:?}", e))
        }
    }
}
