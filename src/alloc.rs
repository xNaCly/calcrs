use crate::types::Type;

const MAX_REGISTERS: usize = 128;

#[derive(Debug)]
pub struct Allocator {
    registers: [bool; MAX_REGISTERS],
}

impl Allocator {
    pub fn new() -> Allocator {
        Allocator {
            registers: [false; MAX_REGISTERS],
        }
    }

    /// alloc stubs the incrementation of the registers used
    pub fn alloc(&mut self) -> Option<usize> {
        for (index, is_allocated) in self.registers.iter().enumerate() {
            if !(*is_allocated) {
                self.registers[index] = true;
                return Some(index + 1);
            }
        }
        None
    }

    /// alloc stubs the decrementation of the registers no longer used
    pub fn dealloc(&mut self, index: usize) {
        if self.registers[index - 1] {
            self.registers[index - 1] = false;
        } else {
            panic!(
                "r{} was not allocated, why is it being deallocated?",
                index + 1
            )
        }
    }
}

#[derive(Debug)]
pub struct Pool {
    pub constants: Vec<Type>,
}

impl Pool {
    pub fn new() -> Pool {
        Pool { constants: vec![] }
    }

    pub fn alloc(&mut self, data: Type) -> usize {
        self.constants.push(data);
        self.constants.len() - 1
    }
}
