/*
 * File: src/parser/token.rs
 * Date: 29.01.2019
 * Auhtor: MarkAtk
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

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    EndOfFile,
    Newline,

    FormatSpecifier,
    ContentSeparator,
    DirectionSeparator,
    Identifier,
    Content,

    LeftGroupParenthesis,
    RightGroupParenthesis,
    LeftTestParenthesis,
    RightTestParenthesis
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: u32,
    pub column: u32
}

impl Token {
    pub fn new(token_type: TokenType) -> Token {
        Token {
            token_type,
            value: String::new(),
            line: 0,
            column: 0
        }
    }

    pub fn new_with_value(token_type: TokenType, value: String, line: u32, column: u32) -> Token {
        Token {
            token_type,
            value,
            line,
            column
        }
    }
}
