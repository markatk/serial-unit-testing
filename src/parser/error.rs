/*
 * File: parser/error.rs
 * Date: 02.10.2018
 * Auhtor: MarkAtk
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

use std::fmt;
use std::error::Error as StdError;

#[derive(PartialEq, Debug)]
pub enum LineError {
    UnknownTextFormat(char),
    InvalidFormat,
    UnallowedCharacter(char, usize),
    UnexpectedEnd
}

impl fmt::Display for LineError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LineError::UnknownTextFormat(ch) => formatter.write_fmt(format_args!("UnknownTextFormat {}", ch)),
            LineError::InvalidFormat => formatter.write_str("InvalidFormat"),
            LineError::UnallowedCharacter(ch, pos) => formatter.write_fmt(format_args!("{} Unallowed character '{}'", pos, ch)),
            LineError::UnexpectedEnd => formatter.write_str("UnexpectedEnd")
        }
    }
}

impl StdError for LineError {
    fn description(&self) -> &str {
        match *self {
            LineError::UnknownTextFormat(_) => "Unknown text format",
            LineError::InvalidFormat => "Invalid line format",
            LineError::UnallowedCharacter(_, _) => "Unallowed character",
            LineError::UnexpectedEnd => "Unexpected end of line"
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ParseError {
    InvalidLine(usize, Option<LineError>)
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidLine(num, e) => {
                if let Some(internal_error) = e {
                    formatter.write_fmt(format_args!("{}:{}", num, internal_error))
                } else {
                    formatter.write_fmt(format_args!("Invalid line {}", num))
                }
            }
        }
    }
}

impl StdError for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::InvalidLine(_, _) => "Invalid line format"
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self {
            ParseError::InvalidLine(_, e) => {
                if let Some(internal_error) = e {
                    Some(internal_error)
                } else {
                    None
                }
            }
        }
    }
}
