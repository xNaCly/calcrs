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
        l.advance();
        return l;
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let t = vec![];
        loop {
            let cc: char;
            if let Some(c) = self.char() {
                cc = c;
            } else {
                break;
            }
            dbg!(cc);
            self.advance();
        }
        t
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos >= self.line.clone().unwrap_or(String::new()).len() || self.line.is_none() {
            self.line = match self.lines.next() {
                Some(line) => Some(line.unwrap()),
                None => None,
            }
        }
    }

    fn char(&mut self) -> Option<char> {
        self.line
            .clone()
            .unwrap_or(String::new())
            .chars()
            .nth(self.pos.try_into().unwrap())
    }
}
