/*
 * File: parser/error.rs
 * Date: 02.10.2018
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

use std::fmt;
use std::error::Error as StdError;

#[derive(Debug, PartialEq)]
pub enum Error {
    Unknown(u32, u32),
    ReadFile,
    IllegalToken(String, u32, u32),
    MissingClosingParenthesis(String, u32, u32),
    MissingDirectionSeparator(u32, u32),
    MissingGroupIdentifier(u32, u32),
    MissingTestIdentifier(u32, u32),
    MissingOptionIdentifier(u32, u32),
    MissingOptionSeparator(u32, u32),
    MissingOptionValue(u32, u32),
    MissingContent(String, u32, u32),
    InvalidLineStart(u32, u32),
    InvalidOptionValue(String, u32, u32),
    InvalidOutputContent(String, u32, u32),
    UnknownTestOption(String, u32, u32),
    UnknownGroupOption(String, u32, u32)
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Unknown(line, column) => formatter.write_fmt(format_args!("Unknown error at {}:{}", line, column)),
            Error::ReadFile => formatter.write_str("Unable to read file"),
            Error::IllegalToken(ref value, line, column) => formatter.write_fmt(format_args!("Illegal token '{}' at {}:{}", value, line, column)),
            Error::MissingClosingParenthesis(ref value, line, column) => formatter.write_fmt(format_args!("Missing closing parenthesis '{}' at {}:{}", value, line, column)),
            Error::MissingDirectionSeparator(line, column) => formatter.write_fmt(format_args!("Missing direction separator at {}:{}", line, column)),
            Error::MissingGroupIdentifier(line, column) => formatter.write_fmt(format_args!("Missing group identifier at {}:{}", line, column)),
            Error::MissingTestIdentifier(line, column) => formatter.write_fmt(format_args!("Missing test identifier at {}:{}", line, column)),
            Error::MissingOptionIdentifier(line, column) => formatter.write_fmt(format_args!("Missing option identifier at {}:{}", line, column)),
            Error::MissingOptionSeparator(line, column) => formatter.write_fmt(format_args!("Missing option separator '=' at {}:{}", line, column)),
            Error::MissingOptionValue(line, column) => formatter.write_fmt(format_args!("Missing option value at {}:{}", line, column)),
            Error::MissingContent(ref content_type, line, column) => formatter.write_fmt(format_args!("Missing test {} at {}:{}", content_type, line, column)),
            Error::InvalidLineStart(line, column) => formatter.write_fmt(format_args!("Invalid line start at {}:{}", line, column)),
            Error::InvalidOptionValue(ref expected_type, line, column) => formatter.write_fmt(format_args!("Invalid option type at {}:{}. {} type expected", line, column, expected_type)),
            Error::InvalidOutputContent(ref content, line, column) => formatter.write_fmt(format_args!("Invalid output content '{}' at {}:{}", content, line, column)),
            Error::UnknownTestOption(ref name, line, column) => formatter.write_fmt(format_args!("Unknown test option '{}' at {}:{}", name, line, column)),
            Error::UnknownGroupOption(ref name, line, column) => formatter.write_fmt(format_args!("Unknown group option '{}' at {}:{}", name, line, column))
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Unknown(_, _) => "Unknown error",
            Error::ReadFile => "File file error",
            Error::IllegalToken(_, _, _) => "Illegal token",
            Error::MissingClosingParenthesis(_, _, _) => "Missing closing parenthesis",
            Error::MissingDirectionSeparator(_, _) => "Missing direction separator",
            Error::MissingGroupIdentifier(_, _) => "Missing group identifier",
            Error::MissingTestIdentifier(_, _) => "Missing test identifier",
            Error::MissingOptionIdentifier(_, _) => "Missing option identifier",
            Error::MissingOptionSeparator(_, _) => "Missing option separator",
            Error::MissingOptionValue(_, _) => "Missing option value",
            Error::MissingContent(_, _, _) => "Missing test content",
            Error::InvalidLineStart(_, _) => "Invalid line start",
            Error::InvalidOptionValue(_, _, _) => "Invalid option value",
            Error::InvalidOutputContent(_, _, _) => "Invalid output content",
            Error::UnknownTestOption(_, _, _) => "Unknown test option",
            Error::UnknownGroupOption(_, _, _) => "Unknown group option"
        }
    }
}
