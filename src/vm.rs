use std::ops::Div;

use crate::{alloc, types::Type};

pub struct Vm {
    registers: Vec<Option<Type>>,
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
            registers: vec![None; 128],
            instructions,
            constants: c.constants.clone(),
        }
    }

    pub fn run(&mut self) {
        if self.instructions.len() % 2 != 0 {
            panic!("Instruction array is invalid");
        }

        for instr_pair in self.instructions.chunks_exact(2) {
            let operation = instr_pair[0];
            let argument = match instr_pair[1] {
                Operation::Argument(v) => v,
                _ => panic!("Wanted an operation of type Argument, got something else"),
            };

            match operation {
                Operation::Load => {
                    let constant = self
                        .constants
                        .get(argument)
                        .unwrap_or_else(|| panic!("Wanted constant at index {}", argument))
                        .clone();
                    self.registers[0] = Some(constant);
                }
                Operation::Store => {
                    let val = self.registers[0].clone();
                    self.registers[argument] = val;
                    self.registers[0] = None;
                }
                Operation::Add | Operation::Sub | Operation::Div | Operation::Mul => {
                    let first = self.registers[argument]
                        .clone()
                        .unwrap_or_else(|| panic!("Invalid register at index {}", argument));
                    let second = self.registers[0].clone().expect("r0 holds no value");

                    let r = match operation {
                        Operation::Add => first.add(second),
                        Operation::Sub => first.sub(second),
                        Operation::Mul => first.mul(second),
                        Operation::Div => first.div(second),
                        _ => panic!("Not supported"),
                    };

                    if r.is_some() {
                        self.registers[0] = r;
                    } else {
                        let first = self.registers[argument].clone().unwrap();
                        let second = self.registers[0].clone().unwrap();
                        panic!(
                            "Can't perform Operation::{:#?} on {:?} and {:?}",
                            operation, first, second
                        );
                    }
                }
                Operation::Debug => {
                    println!(
                        "Operation::Debug at r{}: {:?}",
                        argument,
                        *self
                            .registers
                            .get(argument)
                            .unwrap_or_else(|| panic!("Invalid register at index {}", argument))
                    )
                }
                o => panic!("Operation::{:?} not implemented", o),
            }
        }
    }
}
