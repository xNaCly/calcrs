use crate::{
    expr::{Binary, Constant, Node, Unary, Variable},
    token::{Token, TokenType},
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<Option<Box<dyn Node>>> {
        let mut list: Vec<Option<Box<dyn Node>>> = vec![];
        while !self.at_end() {
            list.push(self.expression());
        }
        list
    }

    fn expression(&mut self) -> Option<Box<dyn Node>> {
        if let TokenType::Ident(_) = self.peek()?.t {
            self.ident()
        } else {
            self.term()
        }
    }

    fn ident(&mut self) -> Option<Box<dyn Node>> {
        let identifier = self.peek()?.t.clone();
        // skip ident
        self.advance();
        // declaration of a variable
        if self.check(TokenType::Equal) {
            // skip equals
            self.advance();
            let rhs = self.primary();
            return Some(Box::new(Variable {
                ident: identifier,
                value: rhs,
            }));
        }
        return Some(Box::new(Constant { t: identifier }));
    }

    fn term(&mut self) -> Option<Box<dyn Node>> {
        let mut lhs = self.factor();
        while self.matches(vec![TokenType::Minus, TokenType::Plus]) {
            let op = self.prev()?.t.clone();
            let rhs = self.factor();
            lhs = Some(Box::new(Binary {
                t: op,
                left: lhs,
                right: rhs,
            }))
        }
        lhs
    }

    fn factor(&mut self) -> Option<Box<dyn Node>> {
        let mut lhs = self.unary();
        while self.matches(vec![TokenType::Slash, TokenType::Asteriks]) {
            let op = self.prev()?.t.clone();
            let rhs = self.unary();
            lhs = Some(Box::new(Binary {
                t: op,
                left: lhs,
                right: rhs,
            }))
        }
        lhs
    }

    fn unary(&mut self) -> Option<Box<dyn Node>> {
        if self.matches(vec![TokenType::Minus]) {
            let rhs = self.unary();
            return Some(Box::new(Unary { right: rhs }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Option<Box<dyn Node>> {
        if let TokenType::String(_) = self.peek()?.t {
            self.advance();
            let op = self.prev()?.t.clone();
            return Some(Box::new(Constant { t: op }));
        } else if let TokenType::Number(_) = self.peek()?.t {
            self.advance();
            let op = self.prev()?.t.clone();
            return Some(Box::new(Constant { t: op }));
        } else if self.matches(vec![TokenType::BraceLeft]) {
            let n = self.expression();
            self.consume(TokenType::BraceRight, "Expected ')'");
            return n;
        }
        dbg!(self.peek());
        panic!("Expected expression")
    }

    fn matches(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, t: TokenType, error: &str) {
        if self.check(t.clone()) {
            return self.advance();
        }
        let tok = self.peek().unwrap();
        panic!("Wanted {:#?}, got {:#?}: {}", t, tok.t, error)
    }

    fn check(&mut self, t: TokenType) -> bool {
        match self.peek() {
            Some(token) => token.t == t,
            None => false,
        }
    }

    fn at_end(&mut self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn advance(&mut self) {
        if !self.at_end() {
            self.pos += 1;
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn prev(&mut self) -> Option<&Token> {
        if self.pos == 0 {
            return None;
        }
        self.tokens.get(self.pos - 1)
    }
}
