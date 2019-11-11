/*
 * File: src/error.rs
 * Date: 11.11.2019
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

use std::error;
use std::fmt;
use std::io;
use std::num;
use std::str;
use std::convert::From;
use serialport;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Serial(serialport::Error),
    Num(num::ParseIntError),
    Utf8(str::Utf8Error),
    Other
}

impl Error {
    pub fn is_timeout(&self) -> bool {
        match *self {
            Error::Io(ref e) if e.kind() == io::ErrorKind::TimedOut => true,
            _ => false
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref cause) => write!(f, "I/O Error: {}", cause),
            Error::Serial(ref cause) => write!(f, "Serial Error: {}", cause),
            Error::Num(ref cause) => write!(f, "Byte Parse Error: {}", cause),
            Error::Utf8(ref cause) => write!(f, "String Parse Error: {}", cause),
            Error::Other => write!(f, "Unknown error")
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref cause) => cause.description(),
            Error::Serial(ref cause) => cause.description(),
            Error::Num(ref cause) => cause.description(),
            Error::Utf8(ref cause) => cause.description(),
            Error::Other => "Unknown error"
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Io(ref cause) => Some(cause),
            Error::Serial(ref cause) => Some(cause),
            Error::Num(ref cause) => Some(cause),
            Error::Utf8(ref cause) => Some(cause),
            Error::Other => None
        }
    }
}

impl From<io::Error> for Error {
    fn from(cause: io::Error) -> Error {
        Error::Io(cause)
    }
}

impl From<serialport::Error> for Error {
    fn from(cause: serialport::Error) -> Error {
        match cause.kind() {
            serialport::ErrorKind::Io(_) => Error::Io(io::Error::from(cause)),
            _ => Error::Serial(cause)
        }
    }
}

impl From<num::ParseIntError> for Error {
    fn from(cause: num::ParseIntError) -> Error {
        Error::Num(cause)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(cause: str::Utf8Error) -> Error {
        Error::Utf8(cause)
    }
}
