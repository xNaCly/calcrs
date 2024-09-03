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
        l.line = l.lines.next().and_then(|l| l.ok());
        l
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
                '(' => tt = Some(TokenType::BraceLeft),
                '=' => tt = Some(TokenType::Equal),
                ')' => tt = Some(TokenType::BraceRight),
                // skip comments
                '#' => {
                    while self.char().is_some() {
                        self.advance();
                    }
                    continue;
                }
                '"' => {
                    self.advance();
                    let start = self.pos;
                    let mut valid_str = false;
                    while let Some(char) = self.char() {
                        if char == '"' {
                            valid_str = true;
                            break;
                        }
                        self.advance();
                    }
                    if !valid_str {
                        panic!("String not closed :(");
                    }

                    // skip closing "
                    self.advance();

                    t.push(Token::new(
                        TokenType::String(String::from(
                            self.line.as_ref().expect("Somehow the line ended before we were done with processing this string").get(start..self.pos-1).unwrap(),
                        )),
                    ));
                    continue;
                }
                '+' => tt = Some(TokenType::Plus),
                '-' => tt = Some(TokenType::Minus),
                '*' => tt = Some(TokenType::Asteriks),
                '/' => tt = Some(TokenType::Slash),
                '0'..='9' => {
                    let start = self.pos;
                    while cc.is_ascii_digit() || cc == '.' || cc == '_' || cc == 'e' {
                        self.advance();
                        if let Some(c) = self.char() {
                            cc = c;
                        } else {
                            break;
                        }
                    }

                    let num = self
                        .line
                        .as_ref()
                        .unwrap_or(&String::new())
                        .get(start..self.pos)
                        .unwrap_or_default()
                        .chars()
                        // rust String.parse does not understand _ in numbers, but i like them :)
                        .filter(|l| *l != '_')
                        .collect::<String>()
                        .parse::<f64>()
                        .expect("Failed to convert input to f64");
                    t.push(Token::new(TokenType::Number(num)));
                    continue;
                }
                _ => {
                    if cc == '_'
                        || cc == '-'
                        || (cc >= 'A' && cc <= 'Z')
                        || (cc >= 'a' && cc <= 'z')
                    {
                        let start = self.pos;
                        while cc.is_ascii_digit()
                            || cc == '_'
                            || cc == '-'
                            || (cc >= 'A' && cc <= 'Z')
                            || (cc >= 'a' && cc <= 'z')
                        {
                            self.advance();
                            if let Some(c) = self.char() {
                                cc = c;
                            } else {
                                break;
                            }
                        }
                        t.push(Token::new(TokenType::Ident(
                            self.line
                                .as_ref()
                                .unwrap_or(&String::new())
                                .get(start..self.pos)
                                .unwrap_or_default()
                                .to_string(),
                        )));
                        continue;
                    }
                    panic!(
                        "Unknown character '{}':{} in line {} and column {}",
                        cc, cc as u32, self.line_count, self.pos
                    )
                }
            }

            match tt {
                Some(ty) => t.push(Token::new(ty)),
                None => panic!("Something happend that shouldn't have"),
            }
            self.advance();
        }
        t
    }

    fn advance_line(&mut self) {
        self.pos = 0;
        self.line_count += 1;
        self.line = self.lines.next().and_then(|l| l.ok());
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    /// char returns the current character
    fn char(&mut self) -> Option<char> {
        self.line.as_ref().and_then(|l| l.chars().nth(self.pos))
    }
}
