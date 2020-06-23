/*
 * File: src/parser/char_util.rs
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

pub fn is_parenthesis(ch: char) -> bool {
    ch == '[' || ch == ']' || ch == '(' || ch == ')'
}

pub fn is_modifier(ch: char) -> bool {
    ch == 'b' || ch == 'o' || ch == 'd' || ch == 'h'
}

pub fn is_separator(ch: char) -> bool {
    ch == ':' || ch == ',' || ch == '='
}

pub fn is_identifier(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '-' || ch == '_' || ch == ' ' || ch == '\t'
}

pub fn is_identifier_start(ch: char) -> bool {
    ch.is_alphanumeric()
}

pub fn is_content_start(ch: char) -> bool {
    ch == '"'
}

pub fn is_newline(ch: char) -> bool {
    ch == '\n' || ch == '\r'
}

pub fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t'
}

pub fn is_comment_start(ch: char) -> bool {
    ch == '#'
}
