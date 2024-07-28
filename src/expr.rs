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
