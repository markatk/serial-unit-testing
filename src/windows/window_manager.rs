/*
 * File: src/windows/window_manager.rs
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

use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::io::{self, stdout, Stdout, Write};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode};
use crossterm::event::{self, KeyEvent, KeyCode, KeyModifiers, read};
use super::{Event, Window};
use crate::windows::EventResult;

pub struct WindowManager {
    windows: Vec<Box<dyn Window>>,
    terminal: Terminal<CrosstermBackend<Stdout>>,
    tx: Sender<Event<KeyEvent>>,
    rx: Receiver<Event<KeyEvent>>,

    pub tick_rate: u64
}

impl WindowManager {
    pub fn new() -> Result<WindowManager, io::Error> {
        let (tx, rx) = mpsc::channel();

        if let Err(err) = enable_raw_mode() {
            return Err(io::Error::new(io::ErrorKind::Other, err));
        }

        let mut stdout = stdout();
        if let Err(err) = execute!(stdout, EnterAlternateScreen) {
            return Err(io::Error::new(io::ErrorKind::Other, err));
        }

        let backend = CrosstermBackend::new(stdout);

        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        Ok(WindowManager {
            windows: vec!(),
            terminal,
            tx,
            rx,
            tick_rate: 500
        })
    }

    pub fn get_tx(&self) -> &Sender<Event<KeyEvent>> {
        &self.tx
    }

    pub fn push_window(&mut self, window: Box<dyn Window>) {
        self.windows.push(window);
    }

    pub fn run(&mut self, initial_window: Box<dyn Window>) -> Result<(), io::Error> {
        self.push_window(initial_window);

        // start input thread
        {
            let tx = self.tx.clone();
            thread::spawn(move || {
                loop {
                    let event = match read() {
                        Ok(event) => event,
                        Err(err) => {
                            println!("Error in input thread: {}", err);

                            return;
                        }
                    };

                    match event {
                        event::Event::Key(key) => {
                            if let Err(_) = tx.send(Event::Input(key.clone())) {
                                return;
                            }
                        },
                        _ => {}
                    }
                }
            });
        }

        // start tick thread
        {
            let tx = self.tx.clone();
            let tick_rate = self.tick_rate;

            thread::spawn(move || {
                loop {
                    tx.send(Event::Tick).unwrap();

                    thread::sleep(std::time::Duration::from_millis(tick_rate));
                }
            });
        }

        // main loop
        while self.windows.is_empty() == false {
            if let Some(window) = self.windows.last_mut() {
                if window.should_close() {
                    self.windows.pop();

                    continue;
                }

                window.render(&mut self.terminal)?;

                let result = match self.rx.recv() {
                    Ok(Event::Input(event)) => {
                        match event {
                            KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::CONTROL } if c == 'c' => return Ok(()),
                            _ => Some(window.handle_key_event(event))
                        }
                    },
                    Ok(Event::Tick) => Some(window.handle_tick(self.tick_rate)),
                    Ok(event) => Some(window.handle_event(event)),
                    _ => None
                };

                if let Some(event_result) = result {
                    self.apply_event_result(event_result);
                }
            };
        }

        Ok(())
    }

    fn apply_event_result(&mut self, event_result: EventResult) {
        if event_result.remove {
            self.windows.pop();
        }

        if let Some(child) = event_result.child {
            self.windows.push(child);
        }
    }
}

impl Drop for WindowManager {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
        self.terminal.show_cursor().unwrap();
    }
}
