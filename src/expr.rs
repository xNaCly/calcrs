use crate::{
    alloc::{Allocator, Pool},
    token::TokenType,
    types::Type,
    vm::Operation,
};

// TODO: implement the Debug trait for the fucking Node trait, rust is weird

pub trait Node {
    fn compile(&self, a: &mut Allocator, c: &mut Pool) -> Vec<Operation>;
}

#[derive(Debug)]
pub struct Constant {
    pub t: TokenType,
}

impl Node for Constant {
    fn compile(&self, _: &mut Allocator, c: &mut Pool) -> Vec<Operation> {
        match &self.t {
            TokenType::Number(number) => {
                let pool_index = c.alloc(Type::Number(*number));
                vec![Operation::Load, Operation::Argument(pool_index)]
            }
            TokenType::String(string) => {
                let pool_index = c.alloc(Type::String(string.to_string()));
                vec![Operation::Load, Operation::Argument(pool_index)]
            }
            _ => panic!("Invalid constant '{:?}'", self.t),
        }
    }
}

pub struct Binary {
    pub t: TokenType,
    pub left: Option<Box<dyn Node>>,
    pub right: Option<Box<dyn Node>>,
}

impl Node for Binary {
    fn compile(&self, a: &mut Allocator, c: &mut Pool) -> Vec<Operation> {
        let mut codes = self
            .left
            .as_ref()
            .map(|l| l.compile(a, c))
            .unwrap_or_default();
        let reg = a.alloc().expect("No more registers available");
        codes.push(Operation::Store);
        codes.push(Operation::Argument(reg));
        let right = selfr.right.as_ref().map(|r| r.compile(a, c));
        if let Some(ops) = right {
            codes.extend(ops);
        }
        let code = match self.t {
            TokenType::Plus => Operation::Add,
            TokenType::Minus => Operation::Sub,
            TokenType::Asteriks => Operation::Mul,
            TokenType::Slash => Operation::Div,
            _ => panic!("Not supported here"),
        };
        codes.push(code);
        codes.push(Operation::Argument(reg));
        a.dealloc(reg);
        codes
    }
}

pub struct Unary {
    pub right: Option<Box<dyn Node>>,
}

impl Node for Unary {
    fn compile(&self, a: &mut Allocator, c: &mut Pool) -> Vec<Operation> {
        let mut codes = self
            .right
            .as_ref()
            .map(|l| l.compile(a, c))
            .unwrap_or_default();
        codes.push(Operation::Neg);
        codes.push(Operation::Argument(0));
        codes
    }
}
