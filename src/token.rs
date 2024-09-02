#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Plus,
    Minus,
    Slash,
    Asteriks,
    BraceLeft,
    BraceRight,
    Equal,
    Number(f64),
    String(String),
    Ident(String),
}

#[derive(Debug)]
pub struct Token {
    pub t: TokenType,
}

impl Token {
    pub fn new(t: TokenType) -> Token {
        Token { t }
    }
}
