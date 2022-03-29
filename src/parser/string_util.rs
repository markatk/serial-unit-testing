/*
 * File: src/parser/string_util.rs
 * Date: 30.01.2019
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

use std::time::Duration;

pub fn get_boolean_value(value: &str) -> Option<bool> {
    match value {
        "false" | "FALSE" => Some(false),
        "true" | "TRUE" => Some(true),
        _ => None
    }
}

pub fn get_time_value(value: &str) -> Option<Duration> {
    let mut time_string = String::new();
    let mut unit_string = String::new();

    // Get <time><unit> strings
    for ch in value.chars() {
        if ch.is_digit(10) && unit_string.is_empty() {
            time_string.push(ch);
        } else if !ch.is_digit(10) && !time_string.is_empty() {
            unit_string.push(ch);
        } else {
            // Wrong format
            return None;
        }
    }

    if time_string.is_empty() {
        return None;
    }

    if let Ok(time) = time_string.parse::<u64>() {
        match unit_string.trim() {
            "us" | "µs" => Some(Duration::from_micros(time)),
            "ms" => Some(Duration::from_millis(time)),
            "s" | "" => Some(Duration::from_secs(time)),
            _ => None
        }
    } else {
        None
    }
}
