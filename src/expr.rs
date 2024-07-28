use crate::{
    alloc::{Allocator, Pool},
    token::TokenType,
    types::Type,
    vm::Operation,
};

pub trait Node {
    fn compile(&self, a: &mut Allocator, c: &mut Pool) -> Vec<Operation>;
}

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

pub struct Binary<'a> {
    pub t: TokenType,
    pub left: &'a dyn Node,
    pub right: &'a dyn Node,
}

impl<'a> Node for Binary<'a> {
    fn compile(&self, a: &mut Allocator, c: &mut Pool) -> Vec<Operation> {
        let mut codes = self.left.compile(a, c);
        let reg = a.alloc().expect("No more registers available");
        codes.push(Operation::Store);
        codes.push(Operation::Argument(reg));
        codes.extend(self.right.compile(a, c).into_iter());
        let code = match self.t {
            TokenType::Plus => Operation::Add,
            TokenType::Sub => Operation::Sub,
            TokenType::Mul => Operation::Mul,
            TokenType::Div => Operation::Div,
            _ => panic!("Not supported here"),
        };
        codes.push(code);
        codes.push(Operation::Argument(reg));
        a.dealloc(reg);
        return codes;
    }
}
