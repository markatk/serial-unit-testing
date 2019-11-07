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
use tui::Terminal;
use tui::backend::CrosstermBackend;
use crossterm::{KeyEvent, InputEvent, input, AlternateScreen};
use super::{Event, Window};

pub struct WindowManager<'a, 'b> {
    windows: Vec<&'a mut dyn Window<'b>>,
    terminal: Terminal<CrosstermBackend>,
    tx: Sender<Event<KeyEvent>>,
    rx: Receiver<Event<KeyEvent>>
}

impl<'a, 'b> WindowManager<'a, 'b> {
    pub fn new() -> Result<WindowManager<'a, 'b>, std::io::Error> {
        let (tx, rx) = mpsc::channel();
        let screen = AlternateScreen::to_alternate(true)?;
        let backend = CrosstermBackend::with_alternate_screen(screen)?;

        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        Ok(WindowManager {
            windows: vec!(),
            terminal,
            tx,
            rx
        })
    }

    pub fn get_tx(&self) -> &Sender<Event<KeyEvent>> {
        &self.tx
    }

    pub fn push_window(&mut self, window: &'a mut dyn Window<'b>) {
        self.windows.push(window);
    }

    pub fn run(&mut self, initial_window: &'a mut dyn Window<'b>) -> Result<(), std::io::Error> {
        self.push_window(initial_window);

        {
            let tx = self.tx.clone();
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
        while self.windows.is_empty() == false {
            if let Some(window) = self.windows.last_mut() {
                if window.should_close() {
                    self.windows.pop();

                    continue;
                }

                window.render(&mut self.terminal)?;

                match self.rx.recv() {
                    Ok(Event::Input(event)) => {
                        // stop execution if true returned
                        match event {
                            KeyEvent::Ctrl(c) => {
                                if c == 'c' {
                                    // Exit application
                                    return Ok(());
                                }
                            },
                            _ => window.handle_key_event(event)
                        };
                    },
                    Ok(event) => window.handle_event(event),
                    _ => ()
                }
            };
        }

        Ok(())
    }
}
