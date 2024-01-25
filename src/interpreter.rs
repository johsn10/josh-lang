use gloo_console::log;
use crate::memory::{self, Memory};
use crate::types::Instruction::{*, self};
use crate::types::Param::*;

pub struct Interpreter {
    pub memory: Memory,
    pub accumulator: i32,
    pub index: usize,
    pub instr_pointer: usize,
}
pub enum InterpreterErr {
    ErrLine(usize),
    Err,
}
pub enum InterpreterResult {
    IndexOveflow,
    NonExistingMem,
    TooLong,
    AddOverflow,
    Ok,
}

impl Interpreter {
    pub fn new(mem_size: usize) -> Self {
        Interpreter {
            memory: Memory::new(mem_size),
            accumulator: 0,
            index: 0,
            instr_pointer: 0,
        }
    }
    pub fn run(&mut self) -> Result<InterpreterResult, InterpreterErr>{
        let max_instruction: i32 = 1000;
        let instructions: Vec<Instruction> = self.memory.instructions.to_vec();
        let mut total_instructions_run: i32 = 0;
        while self.instr_pointer<instructions.len() {
            if total_instructions_run > max_instruction {return Ok(InterpreterResult::TooLong)}
            let instruction = instructions[self.instr_pointer];
            match self.run_instruction(instruction) {
                Ok(result) => {
                    match result {
                        InterpreterResult::Ok => (),
                        InterpreterResult::AddOverflow => return Ok(InterpreterResult::AddOverflow),
                        InterpreterResult::IndexOveflow => return Ok(InterpreterResult::IndexOveflow),
                        InterpreterResult::NonExistingMem => return Ok(InterpreterResult::NonExistingMem),
                        InterpreterResult::TooLong => return Ok(InterpreterResult::TooLong),
                    }
                },
                Err(err) => match err {
                    InterpreterErr::ErrLine(line) => return Err(InterpreterErr::ErrLine(line)),
                    InterpreterErr::Err => return Err(InterpreterErr::ErrLine(self.instr_pointer)),
                },
            };
            total_instructions_run+=1;
        }
        return Ok(InterpreterResult::Ok);
    }
    fn run_instruction(&mut self, instruction: Instruction) -> Result<InterpreterResult, InterpreterErr> {
        log!(Memory::instruction_to_str(instruction));
        match instruction {
            Add(param) => {
                let value: i32 = match param {
                    Const(num) => num,
                    Mem(index) => *self.memory.data.get(index).unwrap(),
                    Index => *self.memory.data.get(self.index).unwrap(),
                    InstructionIndex(_) => return Err(InterpreterErr::Err),  
                };
                match self.accumulator.checked_add(value) {
                    Some(_) => self.accumulator+=value,
                    None => return Ok(InterpreterResult::AddOverflow),
                }
            },
            Store(param) => {
                let index: usize = match param {
                    Mem(index) => index,
                    Const(_) => return Err(InterpreterErr::Err),
                    Index => self.index,
                    InstructionIndex(_) => return Err(InterpreterErr::Err),
                };
                if let Some(value) = self.memory.data.get_mut(index) {
                    *value = self.accumulator;
                } else {return Ok(InterpreterResult::NonExistingMem)}
            },
            Load(param) => {
                let value = match param {
                    Mem(index) => *self.memory.data.get(index).unwrap(),
                    Const(_value) => _value,
                    Index => *self.memory.data.get(self.index).unwrap(),
                    InstructionIndex(_) => return Err(InterpreterErr::Err),
                };
                self.accumulator = value;
            },
            Inc => {
                match self.index.checked_add(1) {
                    Some(value) => self.index = value,
                    None => return Ok(InterpreterResult::IndexOveflow),
                }
            },
            Dec => self.index = self.index.checked_sub(1).unwrap_or(0),
            Jump(param) => {
                match param {
                    Mem(_) => return Err(InterpreterErr::Err),
                    Const(_) => return Err(InterpreterErr::Err),
                    Index => return Err(InterpreterErr::Err),
                    InstructionIndex(instr_pointer) => self.instr_pointer = instr_pointer,
                }
            },
        }
        match instruction {
            Jump(_) => (),
            _ => self.instr_pointer+=1,
        };
        return Ok(InterpreterResult::Ok)
    }
}
