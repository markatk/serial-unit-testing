/*
 * File: src/parser/mod.rs
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

use std::fs;
use std::io::{BufReader, Read};
use regex::Regex;
use crate::tests::{TestCase, TestSuite, TestCaseSettings, TestSuiteSettings};
use crate::utils::TextFormat;

mod error;
mod token;
mod string_util;
mod char_util;
mod lexer;
mod finite_state_machine;
mod options;

use self::lexer::Lexer;
use self::token::{Token, TokenType};
use self::error::Error;
use self::finite_state_machine::FiniteStateMachine;
use self::options::{set_test_option, set_group_option};

/// Parse the given file for tests and test suites.
///
/// A vector of test suites is returned on successful parsing, otherwise a parsing error is returned.
pub fn parse_file(file: &mut fs::File) -> Result<Vec<TestSuite>, Error> {
    parse_file_with_default_settings(file, Default::default())
}

/// Parse the given file for tests and test suites with the given default settings.
///
/// A vector of test suites is returned on successful parsing, otherwise a parsing error is returned.
pub fn parse_file_with_default_settings(file: &mut fs::File, default_test_settings: TestCaseSettings) -> Result<Vec<TestSuite>, Error> {
    let mut reader = BufReader::new(file);
    let mut content = String::new();

    if reader.read_to_string(&mut content).is_err() {
        return Err(Error::ReadFile);
    }

    let mut lexer = Lexer::new(content);
    let tokens = lexer.get_tokens();

    analyse_tokens(tokens, default_test_settings)
}

fn analyse_tokens(tokens: Vec<Token>, default_test_settings: TestCaseSettings) -> Result<Vec<TestSuite>, Error> {
    let mut lines: Vec<Vec<Token>> = Vec::new();
    let mut line: Vec<Token> = Vec::new();

    // split token stream into lines
    for token in tokens {
        if token.token_type == TokenType::Illegal {
            return Err(Error::IllegalToken(token.value, token.line, token.column));
        }

        if token.token_type == TokenType::Newline {
            // only add line if not empty
            if !line.is_empty() {
                lines.push(line);

                line = Vec::new();
            }

            continue;
        }

        line.push(token);
    }

    // analyse each line
    let mut test_suites: Vec<TestSuite> = Vec::new();

    // <> mark optional tokens
    // / mark alternative tokens
    // * repeat tokens
    // [ Identifier <, Identifier < = Value> >* ]
    let group_state_machine = FiniteStateMachine::new(1, vec!(4), |state, token| -> u32 {
        match state {
            1 if token.token_type == TokenType::LeftGroupParenthesis => 2,
            2 if token.token_type == TokenType::Identifier => 3,
            2 if token.token_type == TokenType::ContentSeparator => 5,
            3 if token.token_type == TokenType::RightGroupParenthesis => 4,
            3 if token.token_type == TokenType::ContentSeparator => 5,
            5 if token.token_type == TokenType::Identifier => 6,
            6 if token.token_type == TokenType::OptionSeparator => 7,
            7 if token.token_type == TokenType::Identifier => 3,
            _ => 0
        }
    });

    // <( Identifier <, Identifier < = Value> >* )> < <b/o/d/h>" Content ">* : < <b/o/d/h>" Content ">*
    let test_state_machine = FiniteStateMachine::new(1, vec!(9), |state, token| -> u32 {
        match state {
            1 if token.token_type == TokenType::LeftTestParenthesis => 2,
            1 if token.token_type == TokenType::FormatSpecifier => 5,
            1 if token.token_type == TokenType::Content => 5,
            2 if token.token_type == TokenType::Identifier => 3,
            2 if token.token_type == TokenType::ContentSeparator => 10,
            3 if token.token_type == TokenType::RightTestParenthesis => 4,
            3 if token.token_type == TokenType::ContentSeparator => 10,
            4 if token.token_type == TokenType::FormatSpecifier => 5,
            4 if token.token_type == TokenType::Content => 6,
            5 if token.token_type == TokenType::Content => 6,
            6 if token.token_type == TokenType::DirectionSeparator => 7,
            7 if token.token_type == TokenType::FormatSpecifier => 8,
            7 if token.token_type == TokenType::Content => 9,
            8 if token.token_type == TokenType::Content => 9,
            10 if token.token_type == TokenType::Identifier => 11,
            11 if token.token_type == TokenType::OptionSeparator => 12,
            11 if token.token_type == TokenType::ContentSeparator => 10,
            11 if token.token_type == TokenType::RightTestParenthesis => 4,
            12 if token.token_type == TokenType::Identifier => 3,
            _ => 0
        }
    });

    for line in lines {
        let first_token: &Token = line.first().unwrap();

        if first_token.token_type == TokenType::LeftGroupParenthesis {
            match analyse_test_group(&line, &group_state_machine, default_test_settings.clone()) {
                Ok(test_suite) => test_suites.push(test_suite),
                Err(err) => return Err(err)
            };

            continue;
        }

        if first_token.token_type == TokenType::LeftTestParenthesis || first_token.token_type == TokenType::FormatSpecifier || first_token.token_type == TokenType::Content {
            match analyse_test(&line, &test_state_machine) {
                Ok(test) => {
                    if test_suites.is_empty() {
                        test_suites.push(TestSuite::new(String::new()));
                    }

                    let test_suite: &mut TestSuite = test_suites.last_mut().unwrap();
                    test_suite.push(test);
                }
                Err(err) => return Err(err)
            };

            continue;
        }

        return Err(Error::InvalidLineStart(first_token.line, first_token.column));
    }

    Ok(test_suites)
}

fn analyse_test_group(tokens: &[Token], state_machine: &FiniteStateMachine, default_test_settings: TestCaseSettings) -> Result<TestSuite, Error> {
    let result = state_machine.run(tokens);

    if let Err((state, token)) = result {
        return match state {
            2 => Err(Error::MissingGroupIdentifier(token.line, token.column)),
            3 => Err(Error::MissingClosingParenthesis("]".to_string(), token.line, token.column)),
            5 => Err(Error::MissingOptionIdentifier(token.line, token.column)),
            6 => Err(Error::MissingOptionSeparator(token.line, token.column)),
            7 => Err(Error::MissingOptionValue(token.line, token.column)),
            _ => Err(Error::Unknown(token.line, token.column))
        };
    }

    let mut index = 1;

    let name = if tokens[1].token_type == TokenType::Identifier {
        index += 1;

        tokens[1].value.clone()
    } else {
        String::new()
    };

    let mut settings = TestSuiteSettings::default();
    let mut test_settings = default_test_settings;

    analyse_group_options(&tokens[index..], &mut settings, &mut test_settings)?;

    let test_suite = TestSuite::new_with_settings(name, settings, test_settings);

    Ok(test_suite)
}

fn analyse_test(tokens: &[Token], state_machine: &FiniteStateMachine) -> Result<TestCase, Error> {
    let result = state_machine.run(tokens);

    if let Err((state, token)) = result {
        return match state {
            2 => Err(Error::MissingTestIdentifier(token.line, token.column)),
            3 => Err(Error::MissingClosingParenthesis(")".to_string(), token.line, token.column)),
            4 | 5 => Err(Error::MissingContent("input".to_string(), token.line, token.column)),
            6 => Err(Error::MissingDirectionSeparator(token.line, token.column)),
            7 | 8 => Err(Error::MissingContent("output".to_string(), token.line, token.column)),
            10 => Err(Error::MissingOptionIdentifier(token.line, token.column)),
            11 => Err(Error::MissingOptionSeparator(token.line, token.column)),
            12 => Err(Error::MissingOptionValue(token.line, token.column)),
            _ => Err(Error::Unknown(token.line, token.column))
        };
    }

    // create test case
    let mut name = String::new();
    let mut settings = TestCaseSettings::default();
    let mut input_format: Option<TextFormat> = None;
    let mut output_format: Option<TextFormat> = None;

    let mut index = 0;

    if tokens[index].token_type == TokenType::LeftTestParenthesis {
        if tokens[index + 1].token_type == TokenType::Identifier {
            name = tokens[1].value.clone();
            index += 1;
        }

        index += 1;

        index += analyse_test_options(&tokens[index..], &mut settings)?;
    }

    if tokens[index].token_type == TokenType::FormatSpecifier {
        input_format = Some(get_text_format(&tokens[index])?);
        index += 1;
    }

    let input = tokens[index].value.clone();

    // skip direction separator
    index += 2;

    if tokens[index].token_type == TokenType::FormatSpecifier {
        output_format = Some(get_text_format(&tokens[index])?);
        index += 1;
    }

    let output = tokens[index].value.clone();
    if Regex::new(&output).is_err() {
        return Err(Error::InvalidOutputContent(output, tokens[index].line, tokens[index].column));
    }

    let mut test = TestCase::new(name, input, output);
    test.settings = settings;

    if let Some(format) = input_format {
        test.input_format = format;
    }

    if let Some(format) = output_format {
        test.output_format = format;
    }

    Ok(test)
}

fn analyse_test_options(tokens: &[Token], settings: &mut TestCaseSettings) -> Result<usize, Error> {
    let mut index = 0;

    while tokens[index].token_type == TokenType::ContentSeparator {
        // get length of option
        let mut option_length = 1;
        while tokens[index + option_length].token_type != TokenType::ContentSeparator && tokens[index + option_length].token_type != TokenType::RightTestParenthesis {
            option_length += 1;
        }

        let offset = set_test_option(&tokens[index + 1 .. index + option_length], settings)?;

        index += 2 + offset;
    }

    Ok(index + 1)
}

fn analyse_group_options(tokens: &[Token], settings: &mut TestSuiteSettings, test_settings: &mut TestCaseSettings) -> Result<usize, Error> {
    let mut index = 0;

    while tokens[index].token_type == TokenType::ContentSeparator {
        // get length of option
        let mut option_length = 1;
        while tokens[index + option_length].token_type != TokenType::ContentSeparator && tokens[index + option_length].token_type != TokenType::RightGroupParenthesis {
            option_length += 1;
        }

        // test for both group and test option
        let offset = match set_test_option(&tokens[index + 1 .. index + option_length], test_settings) {
            Ok(offset) => offset,
            Err(err) => {
                match err {
                    Error::UnknownTestOption(_, _, _) => set_group_option(&tokens[index + 1 .. index + option_length], settings)?,
                    _ => return Err(err)
                }
            },
        };

        index += 2 + offset;
    }

    Ok(index + 1)
}

fn get_text_format(token: &Token) -> Result<TextFormat, Error> {
    match token.value.as_str() {
        "b" => Ok(TextFormat::Binary),
        "o" => Ok(TextFormat::Octal),
        "d" => Ok(TextFormat::Decimal),
        "h" => Ok(TextFormat::Hex),
        _ => Err(Error::Unknown(token.line, token.column))
    }
}
