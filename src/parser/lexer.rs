/*
 * File: src/parser/lexer.rs
 * Date: 29.01.2019
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

use super::token::{Token, TokenType};
use super::char_util;

pub struct Lexer {
    input: String,
    position: usize,
    line: u32,
    column: u32
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            position: 0,
            line: 1,
            column: 1
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let token = self.next_token();

            if token.token_type == TokenType::EndOfFile || token.token_type == TokenType::Illegal {
                tokens.push(token);

                break;
            }

            tokens.push(token);
        }

        tokens
    }

    fn next_token(&mut self) -> Token {
        if self.position >= self.input.len() {
            return Token::new(TokenType::EndOfFile);
        }

        self.skip_whitespaces();

        let ch = self.input.chars().nth(self.position).unwrap();

        if char_util::is_comment_start(ch) {
            return self.skip_comment();
        }

        if char_util::is_modifier(ch) {
            if self.position + 1 < self.input.len() && self.input.chars().nth(self.position + 1).unwrap() == '"' {
                return self.recognize_modifier(ch);
            }
        }

        if char_util::is_identifier_start(ch) {
            return self.recognize_identifier();
        }

        if char_util::is_content_start(ch) {
            return self.recognize_content();
        }

        if char_util::is_parenthesis(ch) {
            return self.recognize_parenthesis(ch);
        }

        if char_util::is_separator(ch) {
            return self.recognize_separator(ch);
        }

        if char_util::is_newline(ch) {
            return self.recognize_newline();
        }

        return Token::new_with_value(TokenType::Illegal, ch.to_string(), self.line, self.column);
    }

    fn skip_whitespaces(&mut self) {
        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();

            if char_util::is_whitespace(ch) == false {
                break;
            }

            self.position += 1;
            self.column += 1;
        }
    }

    fn skip_comment(&mut self) -> Token {
        // skip comment start
        self.position += 1;
        self.column += 1;

        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();

            if char_util::is_newline(ch) {
                return self.recognize_newline();
            }

            self.position += 1;
            self.column += 1;
        }

        Token::new(TokenType::Illegal)
    }

    fn recognize_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        let column = self.column;
        
        while self.position <= self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();

            if char_util::is_identifier(ch) == false {
                break;
            }

            identifier.push(ch);

            self.position += 1;
            self.column += 1;
        }

        Token::new_with_value(TokenType::Identifier, identifier, self.line, column)
    }

    fn recognize_content(&mut self) -> Token {
        let mut content = String::new();
        let mut escape_next_char = false;

        let column = self.column;

        // skip starting quotation mark
        self.position += 1;
        self.column += 1;

        while self.position <= self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap();

            if ch == '\\' && escape_next_char == false {
                escape_next_char = true;

                content.push(ch);
                
                self.position += 1;
                self.column += 1;

                continue;
            }

            if ch == '\n' || ch == '\r' {
                return Token::new_with_value(TokenType::Illegal, content, self.line, column);
            }

            if ch == '"' && escape_next_char == false {
                self.position += 1;
                self.column += 1;

                break;
            }

            content.push(ch);

            self.position += 1;
            self.column += 1;
            
            escape_next_char = false;
        }

        Token::new_with_value(TokenType::Content, content, self.line, column)
    }

    fn recognize_separator(&mut self, ch: char) -> Token {
        self.column += 1;
        self.position += 1;

        let token_type = match ch {
            ':' => TokenType::DirectionSeparator,
            ',' => TokenType::ContentSeparator,
            '=' => TokenType::OptionSeparator,
            _ => TokenType::Illegal
        };

        Token::new_with_value(token_type, ch.to_string(), self.line, self.column)
    }

    fn recognize_modifier(&mut self, ch: char) -> Token {
        self.column += 1;
        self.position += 1;

        Token::new_with_value(TokenType::FormatSpecifier, ch.to_string(), self.line, self.column)
    }

    fn recognize_parenthesis(&mut self, ch: char) -> Token {
        self.column += 1;
        self.position += 1;

        let token_type = match ch {
            '[' => TokenType::LeftGroupParenthesis,
            ']' => TokenType::RightGroupParenthesis,
            '(' => TokenType::LeftTestParenthesis,
            ')' => TokenType::RightTestParenthesis,
            _ => TokenType::Illegal
        };

        Token::new_with_value(token_type, ch.to_string(), self.line, self.column)
    }

    fn recognize_newline(&mut self) -> Token {
        let mut value = "\n".to_string();
        let line = self.line;
        let column = self.column;

        self.position += 1;
        self.column = 1;
        self.line += 1;

        if self.position < self.input.len() {
            let next_char = self.input.chars().nth(self.position).unwrap();
            
            if next_char == '\r' {
                value.push('\r');

                self.position += 1;
            }
        }

        Token::new_with_value(TokenType::Newline, value, line, column)
    }
}
