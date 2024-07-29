use crate::{
    expr::{self, Binary, Node},
    token::{self, Token},
};

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Option<Box<dyn Node>> {
        let left = Box::new(expr::Constant {
            t: token::TokenType::String("test".to_string()),
        });
        let right = Box::new(expr::Constant {
            t: token::TokenType::String("test".to_string()),
        });
        Some(Box::new(expr::Binary {
            t: token::TokenType::Plus,
            left: Some(left),
            right: Some(right),
        }))
    }
}
