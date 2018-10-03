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


use std::iter;
use std::str;
use std::fs;
use std::io::{BufReader, BufRead};

use tests::{TestCase, TestSuite, TestCaseSettings};
use utils::TextFormat;

mod error;

pub fn parse_file(file: &mut fs::File) -> Result<Vec<TestSuite>, error::ParseError> {
    let reader = BufReader::new(file);

    let mut test_suites: Vec<TestSuite> = Vec::new();

    for (num, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        let mut iterator = line.chars().enumerate();
        let mut skip_line = false;
        let mut found_group = false;

        loop {
            match iterator.next() {
                Some((_, ' ')) | Some((_, '\t')) => (),
                Some((_, '[')) => {
                    found_group = true;
                    
                    break;
                },
                Some((_, '#')) => {
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
                    Some((_, ']')) => break,
                    Some((_, ch)) => group_name.push(ch),
                    None => return Err(error::ParseError::InvalidLine(num + 1, Some(error::LineError::UnexpectedEnd)))
                };
            }

            test_suites.push(TestSuite::new(group_name));

            continue;
        }

        let test_case = match parse_line(&line) {
            Ok(test_case) => test_case,
            Err(e) => return Err(error::ParseError::InvalidLine(num + 1, Some(e)))
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

pub fn parse_line(line: &str) -> Result<TestCase, error::LineError> {
    let mut iterator = line.chars().enumerate().peekable();

    let input_format = get_text_format(&mut iterator)?;
    let test_name = get_test_name(&mut iterator)?;
    let input = get_formatted_text(&mut iterator, &input_format)?;

    // skip separator
    let mut found_separator = false;

    loop {
        match iterator.peek() {
            Some((_, ' ')) | Some((_, '\t')) => (),
            Some((_, ':')) if found_separator == false => {
                found_separator = true;
            },
            _ if found_separator => break,
            Some((pos, ch)) => return Err(error::LineError::UnallowedCharacter(*ch, *pos)),
            None => return Err(error::LineError::UnexpectedEnd)
        };

        iterator.next();
    }

    let output_format = get_text_format(&mut iterator)?;
    let output = get_formatted_text(&mut iterator, &output_format)?;

    let settings = TestCaseSettings {
        ignore_case: false,
        input_format,
        output_format
    };

    Ok(TestCase::new_with_settings(test_name, input, output, settings))
}

fn get_text_format(iterator: &mut iter::Peekable<iter::Enumerate<str::Chars>>) -> Result<TextFormat, error::LineError> {
    let format = match iterator.peek() {
        Some((_, 'b')) => TextFormat::Binary,
        Some((_, 'o')) => TextFormat::Octal,
        Some((_, 'd')) => TextFormat::Decimal,
        Some((_, 'h')) => TextFormat::Hex,
        Some((_, '"')) | Some((_, '(')) => TextFormat::Text,
        ch @ _ => return Err(error::LineError::UnknownTextFormat(ch.unwrap().1, ch.unwrap().0))
    };

    if format != TextFormat::Text {
        iterator.next();
    }

    Ok(format)
}

fn get_test_name(iterator: &mut iter::Peekable<iter::Enumerate<str::Chars>>) -> Result<String, error::LineError> {
    if let Some((_, ch)) = iterator.peek() {
        if *ch != '(' {
            return Ok(String::new());
        }
    } else {
        return Err(error::LineError::UnexpectedEnd);
    }

    iterator.next();

    let mut name = String::new();

    loop {
        if let Some((_, next_char)) = iterator.next() {
            if next_char == ')' {
                break;
            }

            name.push(next_char);
        } else {
            return Err(error::LineError::UnexpectedEnd);
        }
    }

    Ok(name)
}

fn get_formatted_text(iterator: &mut iter::Peekable<iter::Enumerate<str::Chars>>, text_format: &TextFormat) -> Result<String, error::LineError> {
    if let Some((_, ch)) = iterator.next() {
        if ch != '"' {
            return Err(error::LineError::InvalidFormat);
        }
    } else {
        return Err(error::LineError::UnexpectedEnd);
    }

    let mut text = String::new();
    let mut escape_next_char = false;

    loop {
        let (_, next_char) = iterator.next().unwrap();

        match next_char {
            '"' if escape_next_char == false => break,
            '\\' if *text_format == TextFormat::Text => {
                escape_next_char = true;

                text.push(next_char);

                continue;
            },
            _ => text.push(next_char)
        };

        escape_next_char = false;
    }

    Ok(text)
}
