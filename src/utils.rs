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

pub fn print_radix_string(buffer: &[u8], radix: u32, row_entries: &mut u32, max_row_entries: u32) {
    for b in buffer {
        if radix == 2 {
            print!("{:#b} ", b);
        } else if radix == 8 {
            print!("{:#o}", b);
        } else if radix == 10 {
            print!("{}", b);
        } else if radix == 16 {
            print!("0x{:02X} ", b);
        }

        *row_entries += 1;
        if *row_entries > max_row_entries {
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
