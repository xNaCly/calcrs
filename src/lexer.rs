use std::{
    char,
    fs::File,
    io::{self, BufReader},
};

use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    lines: io::Lines<BufReader<File>>,
    line: Option<String>,
    pos: usize,
}

impl Lexer {
    pub fn new(lines: io::Lines<BufReader<File>>) -> Lexer {
        let mut l = Lexer {
            lines,
            line: None,
            pos: 0,
        };
        l.line = match l.lines.next() {
            Some(line) => Some(line.unwrap()),
            None => None,
        };
        return l;
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let t = vec![];
        loop {
            if self.line.is_none() {
                break;
            }
            let cc: char;
            if let Some(c) = self.char() {
                cc = c;
            } else {
                self.advance_line();
                continue;
            }

            match cc {
                ' ' | '\t' => {
                    self.advance();
                    continue;
                }
                '#' => {
                    while self.char().is_some() {
                        self.advance();
                    }
                    continue;
                }
                _ => (),
            }

            // TODO: add token here

            self.advance();
        }
        t
    }

    fn advance_line(&mut self) {
        self.pos = 0;
        self.line = match self.lines.next() {
            Some(line) => Some(line.unwrap_or_default()),
            None => None,
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn char(&mut self) -> Option<char> {
        self.line
            .clone()
            .unwrap_or_default()
            .chars()
            .nth(self.pos.try_into().unwrap_or_default())
    }
}
