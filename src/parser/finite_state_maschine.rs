/*
 * File: src/parser/finite_state_machine.rs
 * Date: 30.01.2019
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

use super::token::Token;

pub struct FiniteStateMachine {
    initial_state: u32,
    accepting_states: Vec<u32>,
    next_state: fn(u32, &Token) -> u32
}

impl FiniteStateMachine {
    pub fn new(initial_state: u32, accepting_states: Vec<u32>, next_state: fn(u32, &Token) -> u32) -> FiniteStateMachine {
        FiniteStateMachine {
            initial_state,
            accepting_states,
            next_state
        }
    }

    pub fn run<'a>(&self, tokens: &'a Vec<Token>) -> Result<(), (u32, &'a Token)> {
        let mut state = self.initial_state;

        for token in tokens {
            let next_state = (self.next_state)(state, token);

            // TODO: Early return?

            // if no next state stop
            if next_state == 0 {
                return Err((state, token));
            }

            state = next_state;
        }

        if self.accepting_states.contains(&state) == false {
            return Err((state, tokens.last().unwrap()));
        }

        Ok(())
    }
}
