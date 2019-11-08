/*
 * File: src/windows/window.rs
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

use super::Event;

pub trait Window {
    fn setup(&mut self) -> Result<(), std::io::Error> {
        // do nothing
        Ok(())
    }

    fn render(&mut self, terminal: &mut tui::Terminal<tui::backend::CrosstermBackend>) -> Result<(), std::io::Error>;

    fn handle_key_event(&mut self, _event: crossterm::KeyEvent) {
        // do nothing
    }

    fn handle_tick(&mut self, _tick_rate: u64) {
        // do nothing
    }

    fn handle_event(&mut self, _event: Event<crossterm::KeyEvent>) {
        // do nothing
    }

    fn should_close(&self) -> bool;
}