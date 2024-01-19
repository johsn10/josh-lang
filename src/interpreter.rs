use std::fmt::Display;

use crate::memory::Memory;

pub enum Param {
    Mem(usize),
    Const(u8),
    MemIndex,
}

type InstructionIndex = usize;

pub enum Instruction {
    Add(Param),
    Store(Param),
    Load(Param),
    Inc,
    Dec,
    Jump(InstructionIndex)
}
pub struct Interpreter {
    pub memory: Memory,
    pub accumulator: u8,
    pub index: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            memory: Memory::new(),
            accumulator: 0,
            index: 0,
        }
    }
    pub fn run(&mut self, instructions: &Vec<Instruction>) {

    }
}