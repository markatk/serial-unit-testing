/*
 * File: src/parser/options.rs
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

use tests::{TestCaseSettings, TestSuiteSettings};
use super::token::{Token};
use super::string_util;
use super::error::Error;

pub fn set_test_option(tokens: &[Token], settings: &mut TestCaseSettings) -> Result<usize, Error> {
    let name = tokens[0].value.trim();

    // options with implicit value
    match name {
        "ignore-case" => return parse_optional_boolean_option(tokens, &mut settings.ignore_case),
        "allow-failure" => return parse_optional_boolean_option(tokens, &mut settings.allow_failure),
        _ => ()
    };

    // options with explicit value
    if tokens.len() < 3 {
        return Err(Error::MissingOptionValue(tokens[0].line, tokens[0].column));
    }

    let value = tokens[2].value.clone();

    match name {
        "delay" => {
            if let Some(time) = string_util::get_time_value(&value) {
                settings.delay = Some(time);

                Ok(2)
            } else {
                Err(Error::InvalidOptionValue("time".to_string(), tokens[2].line, tokens[2].column))
            }
        },
        "timeout" => {
            if let Some(time) = string_util::get_time_value(&value) {
                settings.timeout = Some(time);

                Ok(2)
            } else {
                Err(Error::InvalidOptionValue("time".to_string(), tokens[2].line, tokens[2].column))
            }
        },
        "repeat" => {
            if let Ok(count) = value.parse::<u32>() {
                settings.repeat = Some(count);

                Ok(2)
            } else {
                Err(Error::InvalidOptionValue("number".to_string(), tokens[2].line, tokens[2].column))
            }
        },
        _ => Err(Error::UnknownTestOption(name.to_string(), tokens[0].line, tokens[0].column))
    }
}

pub fn set_group_option(tokens: &[Token], settings: &mut TestSuiteSettings) -> Result<usize, Error> {
    let name = tokens[0].value.trim();

    // options with implicit value
    match name {
        "stop-on-failure" => return parse_boolean_option(tokens, &mut settings.stop_on_failure),
        _ => ()
    };

    // options with explicit value
    if tokens.len() < 3 {
        return Err(Error::MissingOptionValue(tokens[0].line, tokens[0].column));
    }

    Err(Error::UnknownGroupOption(name.to_string(), tokens[0].line, tokens[0].column))
}

fn parse_boolean_option(tokens: &[Token], option: &mut bool) -> Result<usize, Error> {
    let (value, offset) = if tokens.len() >= 3 {
        (string_util::get_boolean_value(&tokens[2].value.clone()), 2)
    } else {
        (Some(true), 0)
    };

    if let Some(bool_val) = value {
        *option = bool_val;

        Ok(offset)
    } else {
        Err(Error::InvalidOptionValue("boolean".to_string(), tokens[2].line, tokens[2].column))
    }
}

fn parse_optional_boolean_option(tokens: &[Token], option: &mut Option<bool>) -> Result<usize, Error> {
    let mut val = false;
    let offset = parse_boolean_option(tokens, &mut val)?;

    *option = Some(val);

    Ok(offset)
}
