/*
 * File: src/utils.rs
 * Date: 30.09.2018
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

#[derive(PartialEq, Debug)]
pub enum TextFormat {
    Text,
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hex = 16
}

pub fn bytes_from_hex_string(original_text: &str) -> Result<Vec<u8>, String> {
    let mut text = original_text.replace("0x", "");
    text = text.replace(" ", "");

    bytes_from_radix_string(&text, 16)
}

pub fn bytes_from_binary_string(orignal_text: &str) -> Result<Vec<u8>, String> {
    let mut text = orignal_text.replace("0b", "");
    text = text.replace(" ", "");

    bytes_from_radix_string(&text, 2)
}

pub fn bytes_from_radix_string(text: &str, radix: u32) -> Result<Vec<u8>, String> {
    let mut bytes: Vec<u8> = Vec::new();

    let mut chars = text.chars().peekable();

    while chars.peek().is_some() {
        let chunk: String = chars.by_ref().take(2).collect();

        match u8::from_str_radix(&chunk, radix) {
            Ok(value) => bytes.push(value),
            Err(e) => return Err(format!("Unable to read input string {}", e))
        };
    }

    Ok(bytes)
}

pub fn radix_string<'a>(buffer: &'a [u8], text_format: &TextFormat) -> String {
    let mut text = String::new();

    for b in buffer {
        match text_format {
            TextFormat::Binary => text.push_str(format!("{:b}", b).as_str()),
            TextFormat::Octal => text.push_str(format!("{:o}", b).as_str()),
            TextFormat::Decimal => text.push_str(format!("{}", b).as_str()),
            TextFormat::Hex => text.push_str(format!("{:02X}", b).as_str()),
            _ => ()
        };
    }

    text
}

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

pub fn escape_text(text: String) -> String {
    let mut text = text.replace("\\r", "\r");
    text = text.replace("\\n", "\n");
    text = text.replace("\\t", "\t");

    text
}
