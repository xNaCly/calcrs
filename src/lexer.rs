use std::{
    char,
    fs::File,
    io::{self, BufReader},
};

use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    lines: io::Lines<BufReader<File>>,
    line: Option<String>,
    pos: usize,
    line_count: usize,
}

impl Lexer {
    pub fn new(lines: io::Lines<BufReader<File>>) -> Lexer {
        let mut l = Lexer {
            lines,
            line: None,
            pos: 0,
            line_count: 1,
        };
        l.line = match l.lines.next() {
            Some(line) => Some(line.unwrap()),
            None => None,
        };
        return l;
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut t = vec![];
        loop {
            // if no more line available, just stop
            if self.line.is_none() {
                break;
            }

            let mut cc: char;
            if let Some(c) = self.char() {
                cc = c;
            } else {
                // we will hit this once we cant advance anymore, means we are at the end of the
                // curent line
                self.advance_line();
                continue;
            }

            let tt: Option<TokenType>;
            match cc {
                // skip whitespace
                ' ' | '\t' => {
                    self.advance();
                    continue;
                }
                // skip comments
                '#' => {
                    while self.char().is_some() {
                        self.advance();
                    }
                    continue;
                }
                '+' => tt = Some(TokenType::PLUS),
                '-' => tt = Some(TokenType::SUB),
                '*' => tt = Some(TokenType::MUL),
                '/' => tt = Some(TokenType::DIV),
                '0'..='9' => {
                    let start = self.pos;
                    while cc.is_digit(10) || cc == '.' || cc == '_' || cc == 'e' {
                        self.advance();
                        if let Some(c) = self.char() {
                            cc = c;
                        } else {
                            break;
                        }
                    }

                    let num = self
                        .line
                        .clone()
                        .unwrap_or_default()
                        .get(start..self.pos)
                        .unwrap_or_default()
                        .chars()
                        // rust String.parse does not understand _ in numbers, but i like them :)
                        .filter(|l| *l != '_')
                        .collect::<String>()
                        .parse::<f64>()
                        .expect("Failed to convert input to f64");
                    t.push(Token::new(start, TokenType::NUMBER(num)));
                    continue;
                }
                _ => {
                    panic!(
                        "Unknown character '{}':{} in line {} and column {}",
                        cc, cc as u32, self.line_count, self.pos
                    )
                }
            }

            match tt {
                Some(ty) => t.push(Token::new(self.pos, ty)),
                None => panic!("Something happend that shouldn't have"),
            }
            self.advance();
        }
        t
    }

    fn advance_line(&mut self) {
        self.pos = 0;
        self.line_count += 1;
        self.line = match self.lines.next() {
            Some(line) => Some(line.unwrap_or_default()),
            None => None,
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    /// char returns the current character
    fn char(&mut self) -> Option<char> {
        self.line
            .clone()
            .unwrap_or_default()
            .chars()
            .nth(self.pos.try_into().unwrap_or_default())
    }
}
