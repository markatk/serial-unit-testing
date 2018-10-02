/*
 * File: src/parser/mod.rs
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
use std::iter;
use std::str;

use tests::{TestCase};
use utils::TextFormat;

#[derive(Debug)]
pub enum Error {
    UnknownTextFormat(char),
    InvalidFormat,
    UnallowedCharacter(char)
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnknownTextFormat(_) => formatter.write_str("UnknownTextFormat"),
            Error::InvalidFormat => formatter.write_str("InvalidFormat"),
            Error::UnallowedCharacter(_) => formatter.write_str("UnallowedCharacter")
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnknownTextFormat(_) => "Unknown text format",
            Error::InvalidFormat => "Invalid line format",
            Error::UnallowedCharacter(_) => "Unallowed character"
        }
    }
}

pub fn parse_line(line: &str) -> Result<TestCase, Error> {
    let mut iterator = line.chars().peekable();

    let input_format = get_text_format(&mut iterator)?;
    let _input_name = get_test_name(&mut iterator)?;
    let (_input, _raw_input) = get_formatted_text(&mut iterator, &input_format)?;

    Err(Error::InvalidFormat)
}

fn get_text_format(iterator: &mut iter::Peekable<str::Chars>) -> Result<TextFormat, Error> {
    let ch = iterator.peek().cloned();

    let format = match ch {
        Some('b') => TextFormat::Binary,
        Some('o') => TextFormat::Octal,
        Some('d') => TextFormat::Decimal,
        Some('h') => TextFormat::Hex,
        Some('"') | Some('(') => TextFormat::Text,
        _ => return Err(Error::UnknownTextFormat(ch.unwrap()))
    };

    if format != TextFormat::Text {
        iterator.next();
    }

    Ok(format)
}

fn get_test_name(iterator: &mut iter::Peekable<str::Chars>) -> Result<String, Error> {
    if *iterator.peek().unwrap() != '(' {
        return Ok(String::new());
    }

    iterator.next();

    let mut name = String::new();

    loop {
        let next_char = iterator.next().unwrap();

        if next_char == ')' {
            break;
        }

        name.push(next_char);
    }

    Ok(name)
}

fn get_formatted_text(iterator: &mut iter::Peekable<str::Chars>, text_format: &TextFormat) -> Result<(String, String), Error> {
    if iterator.next().unwrap() != '"' {
        return Err(Error::InvalidFormat);
    }

    let mut text = String::new();
    let mut raw_text = String::new();
    let mut escape_next_char = false;

    loop {
        let mut next_char = iterator.next().unwrap();

        if (escape_next_char == false && next_char == '"') == false{
            raw_text.push(next_char);
        }

        if escape_next_char {
            next_char = match next_char {
                't' => '\t',
                'n' => '\n',
                'r' => '\r',
                _ => next_char
            }
        } else {
            match next_char {
                '"' => break,
                '\\' if *text_format == TextFormat::Text => {
                    escape_next_char = true;

                    continue;
                },
                _ => ()
            };
        }

        escape_next_char = false;

        text.push(next_char);
    }

    Ok((text, raw_text))
}

