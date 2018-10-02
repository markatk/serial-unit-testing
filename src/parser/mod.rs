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
use std::fs;
use std::io::{BufReader, BufRead};

use tests::{TestCase, TestSuite, TestCaseSettings};
use utils::TextFormat;

#[derive(Debug)]
pub enum Error {
    UnknownTextFormat(char),
    InvalidFormat,
    UnallowedCharacter(char),
    InvalidLine(usize)
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnknownTextFormat(ch) => formatter.write_fmt(format_args!("UnknownTextFormat {}", ch)),
            Error::InvalidFormat => formatter.write_str("InvalidFormat"),
            Error::UnallowedCharacter(ch) => formatter.write_fmt(format_args!("UnallowedCharacter {}", ch)),
            Error::InvalidLine(line) => formatter.write_fmt(format_args!("InvalidLine {}", line))
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnknownTextFormat(_) => "Unknown text format",
            Error::InvalidFormat => "Invalid line format",
            Error::UnallowedCharacter(_) => "Unallowed character",
            Error::InvalidLine(_) => "Invalid line format"
        }
    }
}

pub fn parse_file(file: &mut fs::File) -> Result<Vec<TestSuite>, Error> {
    let reader = BufReader::new(file);

    let mut test_suites: Vec<TestSuite> = Vec::new();

    for (num, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        let mut iterator = line.chars();
        let mut skip_line = false;
        let mut found_group = false;

        loop {
            match iterator.next() {
                Some(' ') | Some('\t') => (),
                Some('[') => found_group = true,
                Some('#') => {
                    skip_line = true;

                    break;
                },
                _ => break
            };
        }

        if skip_line {
            continue;
        }

        if found_group {
            let mut group_name = String::new();

            loop {
                match iterator.next() {
                    Some(']') => break,
                    Some(ch) => group_name.push(ch),
                    None => return Err(Error::InvalidLine(num))
                };
            }

            test_suites.push(TestSuite::new(group_name));

            continue;
        }

        let test_case = match parse_line(&line) {
            Ok(test_case) => test_case,
            //Err(_) => return Err(Error::InvalidLine(num))
            Err(e) => {
                println!("{}", e);

                return Err(Error::InvalidLine(num + 1));
            }
        };

        if test_suites.len() > 0 {
            test_suites.last_mut().unwrap().push(test_case);
        } else {
            let mut test_suite = TestSuite::new(String::new());
            test_suite.push(test_case);

            test_suites.push(test_suite);
        }
    }

    Ok(test_suites)
}

pub fn parse_line(line: &str) -> Result<TestCase, Error> {
    let mut iterator = line.chars().peekable();

    let input_format = get_text_format(&mut iterator)?;
    let _input_name = get_test_name(&mut iterator)?;
    let (input, raw_input) = get_formatted_text(&mut iterator, &input_format)?;

    // skip separator
    let mut found_separator = false;

    loop {
        match iterator.peek() {
            Some(' ') | Some('\t') => (),
            Some(':') if found_separator == false => {
                found_separator = true;
            },
            Some(ch) => return Err(Error::UnallowedCharacter(*ch)),
            _ if found_separator => break,
            None => return Err(Error::InvalidFormat)
        };

        iterator.next();
    }

    let output_format = get_text_format(&mut iterator)?;
    let (output, raw_output) = get_formatted_text(&mut iterator, &output_format)?;

    let settings = TestCaseSettings {
        ignore_case: false,
        input_format,
        output_format
    };

    Ok(TestCase::new_with_settings(input, raw_input, output, raw_output, settings))
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

