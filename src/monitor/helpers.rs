/*
 * File: src/monitor/helpers.rs
 * Date: 29.10.2019
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

use serial_unit_testing::utils::{TextFormat, NewlineFormat};

// TODO: Move all of this into utils
pub fn char_count(str: &String) -> usize {
    str.char_indices().count()
}

pub fn byte_position_of_char(str: &String, index: usize) -> Option<usize> {
    match str.char_indices().nth(index) {
        Some((pos, _)) => Some(pos),
        None => None
    }
}

pub fn remove_char(str: &mut String, index: usize) {
    if let Some(pos) = byte_position_of_char(str, index) {
        str.remove(pos);
    };
}

pub fn insert_char(str: &mut String, index: usize, c: char) {
    if let Some(pos) = byte_position_of_char(str, index) {
        str.insert(pos, c);
    }
}

pub fn get_format_name(format: &TextFormat) -> &str {
    match format {
        TextFormat::Text => "Text",
        TextFormat::Binary => "Binary",
        TextFormat::Octal => "Octal",
        TextFormat::Decimal => "Decimal",
        TextFormat::Hex => "Hexadecimal"
    }
}

pub fn get_next_format(format: &TextFormat) -> TextFormat {
    match format {
        TextFormat::Text => TextFormat::Binary,
        TextFormat::Binary => TextFormat::Octal,
        TextFormat::Octal => TextFormat::Decimal,
        TextFormat::Decimal => TextFormat::Hex,
        TextFormat::Hex => TextFormat::Text
    }
}

pub fn get_newline_format_name(format: &NewlineFormat) -> &str {
    match format {
        NewlineFormat::None => "None",
        NewlineFormat::CarriageReturn => "Carriage return",
        NewlineFormat::LineFeed => "Line feed",
        NewlineFormat::Both => "Both"
    }
}

pub fn get_next_newline_format(format: &NewlineFormat) -> NewlineFormat {
    match format {
        NewlineFormat::None => NewlineFormat::CarriageReturn,
        NewlineFormat::CarriageReturn => NewlineFormat::LineFeed,
        NewlineFormat::LineFeed => NewlineFormat::Both,
        NewlineFormat::Both => NewlineFormat::None
    }
}

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

pub fn get_bool(value: bool) -> &'static str {
    match value {
        true => "Yes",
        false => "No"
    }
}