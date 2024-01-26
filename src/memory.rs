use gloo_console::log;
use substring::Substring;
use regex::Regex;
use std::iter::Iterator;
use crate::types::Instruction::{*, self};
use crate::types::Param::{*, self};

#[derive(Clone)]
pub struct Memory {
    pub data: Vec<i32>,
    pub instructions: Vec<Instruction>,
}

impl Memory {
    pub fn new(memory_size: usize) -> Self {
        Memory {
            data: vec![0_i32;memory_size],
            instructions: vec![],
        }
    }
    pub fn store_instruction_str(&mut self, instructions_str: &str) -> Result<(), usize> {
        let re = Regex::new(r"(?<instruction>Add|Store|Load|Jump|Inc|Dec) ?((?<const>-?\d+)|(?<mem>@\d+)|(?<index>@Index)|(?<instruction_index>#\d+)|(?<no_param>))").unwrap();
        let mut instructions: Vec<Instruction> = vec![];
        for (line_index, instruction_line) in instructions_str.lines().enumerate() {
            let instruction: Instruction;

            let Some(caps) = re.captures(instruction_line) else {
                return Err(line_index);
            };
            
            let instruction_str: &str;
            if let Some(_instruction) = caps.name("instruction") {
                instruction_str = _instruction.as_str();
            } else {return Err(line_index)}
            
            let param: Param;
            if let Some(_param) = caps.name("const") {
                let value: i32;
                if let Ok(_value) = _param.as_str().parse::<i32>() {
                    value = _value;
                    param = Const(value);
                } else {return Err(line_index)}
            }
            else if let Some(_param) = caps.name("mem") {
                let param_str = _param.as_str();
                let value_str = param_str.substring(1, param_str.len());
                let value: usize;
                if let Ok(_value) = value_str.parse::<usize>() {
                    value = _value;
                } else {return Err(line_index)}
                param = Mem(value);
            }
            else if let Some(_param) = caps.name("index") {
                param = Index;
            }
            else if let Some(_param) = caps.name("instruction_index") {
                let param_str = _param.as_str();
                let value_str = param_str.substring(1, param_str.len());
                let value: usize;
                if let Ok(_value) = value_str.parse::<usize>() {
                    value = _value;
                } else {return Err(line_index)}
                param = InstructionIndex(value);
            }
            else if caps.name("no_param").is_some() && (instruction_str=="Dec" || instruction_str=="Inc") {
                param=Const(0);
            }
            else {return Err(line_index)};
            instruction = Self::str_to_instruction(&instruction_str, &param);
            instructions.push(instruction);
        }
        self.store_instructions(instructions);
        return Ok(());
    }

    fn str_to_instruction(instruction_str: &str, param: &Param) -> Instruction {
        match instruction_str {
            "Add" => Add(*param),
            "Store" => Store(*param),
            "Load" => Load(*param),
            "Inc" => Inc,
            "Dec" => Dec,
            "Jump" => Jump(*param),
            _ => panic!()
        }
    }

    pub fn instruction_to_str(instruction: Instruction) -> String {
        match instruction {
            Add(param) => format!("Add({})", Self::param_to_str(param)),
            Store(param) => format!("Store({})", Self::param_to_str(param)),
            Load(param) => format!("Load({})", Self::param_to_str(param)),
            Inc => "Inc".to_owned(),
            Dec => "Dec".to_owned(),
            Jump(param) => format!("Jump({})", Self::param_to_str(param)),
        }
    }
    fn param_to_str(param: Param) -> String {
        match param {
            Mem(v) => format!("Mem({})", v),
            Const(v) => format!("Const({})", v),
            Index => format!("Index"),
            InstructionIndex(v) => format!("InstructionIndex({})", v),
        }
    }
    pub fn store_instructions(&mut self, instructions: Vec<Instruction>) {
        self.instructions = instructions;
    }
    pub fn log_memory(&self) {
        let data = &self.data;
        let mut result = String::new();
        for num in data {
            result = format!("{}, {}", result, num);
        }
        log!(result);
    }
}