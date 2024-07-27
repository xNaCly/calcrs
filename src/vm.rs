use crate::{alloc, types::Type};

pub struct Vm {
    registers: [Option<Type>; 128],
    constants: Vec<Type>,
    instructions: Vec<Operation>,
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Sub,
    Div,
    Mul,
    /// loads argument into r0
    Load,
    /// stores value of r0 in register specified in argument
    Store,
    /// prints the value stored in the register specified in the argument
    Debug,
    Argument(usize),
}

impl Vm {
    pub fn new(c: &alloc::Pool, instructions: Vec<Operation>) -> Vm {
        Vm {
            registers: [None; 128],
            instructions,
            constants: c.constants.clone(),
        }
    }

    pub fn run(&mut self) {
        if self.instructions.len() % 2 != 0 {
            panic!("Instruction array is invalid");
        }
        let mut i = 0;
        while i < self.instructions.len() {
            let operation = self.instructions.get(i).expect("Failed to get Operation");
            let raw_argument = self
                .instructions
                .get(i + 1)
                .expect("Failed to get Operation argument");
            let argument = match raw_argument {
                Operation::Argument(v) => *v,
                _ => panic!("Wanted an operation of type Argument, got something else"),
            };
            match operation {
                Operation::Load => {
                    let constant = self
                        .constants
                        .get(argument)
                        .expect(&format!("Wanted constant at index {}", argument));
                    self.registers[0] = Some(*constant);
                }
                Operation::Debug => {
                    println!(
                        "Operation::Debug at r{}: {:?}",
                        argument,
                        *self.registers.get(argument).unwrap()
                    )
                }
                o => panic!("{:?} not implemented", o),
            }
            i += 2;
        }
    }
}
