/*
 * File: src/utils.rs
 * Date: 30.09.2018
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

use std::str;
use super::error::{Error, Result};

/// Text format type for radix string conversion.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TextFormat {
    /// Text format
    Text,
    /// Binary format
    Binary = 2,
    /// Octal format
    Octal = 8,
    /// Decimal format
    Decimal = 10,
    /// Hexadecimal format
    Hex = 16
}

/// Get a text representation of the text format.
pub fn get_format_name(format: &TextFormat) -> &str {
    match format {
        TextFormat::Text => "Text",
        TextFormat::Binary => "Binary",
        TextFormat::Octal => "Octal",
        TextFormat::Decimal => "Decimal",
        TextFormat::Hex => "Hexadecimal"
    }
}

/// Get the next cyclic text format.
///
/// If the last value is passed into this, the first value will be returned.
pub fn get_next_format(format: &TextFormat) -> TextFormat {
    match format {
        TextFormat::Text => TextFormat::Binary,
        TextFormat::Binary => TextFormat::Octal,
        TextFormat::Octal => TextFormat::Decimal,
        TextFormat::Decimal => TextFormat::Hex,
        TextFormat::Hex => TextFormat::Text
    }
}

/// Newline format type
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NewlineFormat {
    None,
    CarriageReturn,
    LineFeed,
    Both
}

/// Get a text representation of the newline format.
pub fn get_newline_format_name(format: &NewlineFormat) -> &str {
    match format {
        NewlineFormat::None => "None",
        NewlineFormat::CarriageReturn => "Carriage return",
        NewlineFormat::LineFeed => "Line feed",
        NewlineFormat::Both => "Both"
    }
}

/// Get the next cyclic newline format.
///
/// If the last value is passed into this, the first value will be returned.
pub fn get_next_newline_format(format: &NewlineFormat) -> NewlineFormat {
    match format {
        NewlineFormat::None => NewlineFormat::CarriageReturn,
        NewlineFormat::CarriageReturn => NewlineFormat::LineFeed,
        NewlineFormat::LineFeed => NewlineFormat::Both,
        NewlineFormat::Both => NewlineFormat::None
    }
}

/// Convert a hexadecimal string into a vector of bytes.
///
/// Leading 0x and whitespaces will be ignored.
pub fn bytes_from_hex_string(original_text: &str) -> Result<Vec<u8>> {
    let mut text = original_text.replace("0x", "");
    text = text.replace(" ", "");

    bytes_from_radix_string(&text, 16)
}

/// Convert a binary string into a vector of bytes.
///
/// Leading 0b and whitespaces will be ignored.
pub fn bytes_from_binary_string(original_text: &str) -> Result<Vec<u8>> {
    let mut text = original_text.replace("0b", "");
    text = text.replace(" ", "");

    bytes_from_radix_string(&text, 2)
}

/// Convert a octal string into a vector of bytes
///
/// Leading 0 and whitespaces will be ignored.
pub fn bytes_from_octal_string(original_text: &str) -> Result<Vec<u8>> {
    let mut text = original_text.replace(" ", "");
    if text.starts_with('0') {
        text.remove(0);
    }

    bytes_from_radix_string(&text, 8)
}

/// Convert a decimal string into a vector of bytes
///
/// Whitespaces will be ignored
pub fn bytes_from_decimal_string(original_text: &str) -> Result<Vec<u8>> {
    let text = original_text.replace(" ", "");

    bytes_from_radix_string(&text, 10)
}

/// Convert a radix string into a vector of bytes.
///
/// Leading and trailing whitespaces will result in an error. Conversion happens by 2 characters per byte.
pub fn bytes_from_radix_string(text: &str, radix: u32) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();

    let mut chars = text.chars().peekable();

    while chars.peek().is_some() {
        // TODO: Fix single char radix (when count is odd)
        let chunk: String = chars.by_ref().take(2).collect();

        match u8::from_str_radix(&chunk, radix) {
            Ok(value) => bytes.push(value),
            Err(e) => return Err(Error::from(e))
        };
    }

    Ok(bytes)
}

/// Convert a vector of bytes into a radix string with given text format.
pub fn radix_string(buffer: &[u8], text_format: &TextFormat) -> Result<String> {
    if *text_format == TextFormat::Text {
        return match str::from_utf8(buffer) {
            Ok(text) => Ok(text.to_string()),
            Err(e) => Err(Error::from(e))
        };
    }

    let mut text = String::new();

    for b in buffer {
        match text_format {
            TextFormat::Binary => text.push_str(format!("{:08b}", b).as_str()),
            TextFormat::Octal => text.push_str(format!("{:04o}", b).as_str()),
            TextFormat::Decimal => text.push_str(format!("{}", b).as_str()),
            TextFormat::Hex => text.push_str(format!("{:02X}", b).as_str()),
            _ => ()
        };
    }

    Ok(text)
}

/// Print a vector of bytes in given format.
///
/// Depending on the format newlines will be inserted after some entries as followed:
/// - Binary: 10
/// - Octal: 16
/// - Decimal: 18
/// - Hexadecimal: 20
///
/// Row entries will contain the number of entries in the last row after execution.
pub fn print_radix_string(buffer: &[u8], text_format: &TextFormat, row_entries: &mut u32) {
    let max_row_entries = match text_format {
        TextFormat::Binary => 10,
        TextFormat::Octal => 16,
        TextFormat::Decimal => 18,
        TextFormat::Hex => 20,
        _ => 0
    };

    for b in buffer {
        match text_format {
            TextFormat::Binary => print!("{:#b} ", b),
            TextFormat::Octal => print!("{:#o} ", b),
            TextFormat::Hex => print!("0x{:02X} ", b),
            _ => print!("{}", b)
        };

        *row_entries += 1;
        if max_row_entries > 0 && *row_entries > max_row_entries {
            *row_entries = 0;

            println!();
        }
    }
}

/// Escape given string.
///
/// Characters escaped are: \r, \n, \t
pub fn escape_text(text: String) -> String {
    let mut text = text.replace("\\r", "\r");
    text = text.replace("\\n", "\n");
    text = text.replace("\\t", "\t");
    text = text.replace("\\\"", "\"");

    text
}

/// Get actual character count (instead of byte count).
pub fn char_count(str: &String) -> usize {
    str.char_indices().count()
}

/// Get the starting position of a character in a string.
///
/// When working with unicode characters a character can span multiple bytes and thus
/// the offset of characters is not the same as the number of bytes.
pub fn byte_position_of_char(str: &String, index: usize) -> Option<usize> {
    match str.char_indices().nth(index) {
        Some((pos, _)) => Some(pos),
        None => None
    }
}

/// Remove a character by given character index (instead of byte position).
pub fn remove_char(str: &mut String, index: usize) {
    if let Some(pos) = byte_position_of_char(str, index) {
        str.remove(pos);
    };
}

/// Insert a character by given character index (instead of byte position).
pub fn insert_char(str: &mut String, index: usize, c: char) {
    if let Some(pos) = byte_position_of_char(str, index) {
        str.insert(pos, c);
    }
}

/// Add newline characters to the end of the string.
///
/// The newline characters will be added in the given text format.
pub fn add_newline(text: &mut String, text_format: TextFormat, newline_format: NewlineFormat) {
    if newline_format == NewlineFormat::None {
        return;
    }

    let cr = match text_format {
        TextFormat::Text => "\r",
        TextFormat::Binary => "00001101",
        TextFormat::Octal => "015",
        TextFormat::Decimal => "13",
        TextFormat::Hex => "0D"
    };

    let lf = match text_format {
        TextFormat::Text => "\n",
        TextFormat::Binary => "00001010",
        TextFormat::Octal => "012",
        TextFormat::Decimal => "10",
        TextFormat::Hex => "0A"
    };

    if newline_format == NewlineFormat::CarriageReturn || newline_format == NewlineFormat::Both {
        text.push_str(cr);
    }

    if newline_format == NewlineFormat::LineFeed || newline_format == NewlineFormat::Both {
        text.push_str(lf);
    }
}
