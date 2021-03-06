/*
 * File: src/windows/mod.rs
 * Date: 01.11.2019
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

mod window;
mod window_manager;

pub use window::Window;
pub use window_manager::WindowManager;

#[derive(Debug, Clone)]
pub struct WindowError {
    pub description: String,
    pub recoverable: bool
}

impl WindowError {
    pub fn new(description: String, recoverable: bool) -> WindowError {
        WindowError {
            description,
            recoverable
        }
    }
}

pub enum Event<I> {
    Input(I),
    Tick,
    Output(Vec<u8>),
    Error(WindowError)
}

pub struct EventResult {
    pub remove: bool,
    pub child: Option<Box<dyn Window>>
}

impl EventResult {
    pub fn new() -> EventResult {
        EventResult {
            remove: false,
            child: None
        }
    }
}
