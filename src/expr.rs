use crate::{
    alloc::{Allocator, Pool},
    token::TokenType,
    types::Type,
    vm::Operation,
};

pub trait Node {
    fn compile(&self, a: &mut Allocator, c: &mut Pool) -> Vec<Operation>;
}

pub struct Number {
    pub t: TokenType,
}

impl Node for Number {
    fn compile(&self, a: &mut Allocator, c: &mut Pool) -> Vec<Operation> {
        if let TokenType::NUMBER(number) = self.t {
            let pool_index = c.alloc(Type::Number(number));
            vec![Operation::Load, Operation::Argument(pool_index)]
        } else {
            panic!(
                "This should not have happened, hit a token of type Number without a number inside"
            );
        }
    }
}
