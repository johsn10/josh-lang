use std::fmt::format;

use gloo_console::log;
use substring::Substring;
use wasm_bindgen::JsValue;
use regex::Regex;

use crate::interpreter::Instruction::{*, self};
use crate::interpreter::Param::{*, self};

pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: vec![],
        }
    }
    pub fn instructions_string_to_instructions(&mut self, instructions_str: &str) -> Option<()> {
        let re = Regex::new(r"(?<instruction>Add|Store|Load|Jump|Inc|Dec) ((?<const>\d+)|(?<mem>@\d+)|(?<index>@Index)|(?<instruction_index>#\d+))").unwrap();
        let mut instructions: Vec<Instruction> = vec![];
        for instruction_line in instructions_str.lines() {
            let mut instruction: Instruction;

            let Some(caps) = re.captures(instruction_line) else {
                return None;
            };
            
            let instruction_str: &str;
            if let Some(_instruction) = caps.name("instruction") {
                instruction_str = _instruction.as_str();
            } else {return None}
            
            let param: Param;
            if let Some(_param) = caps.name("const") {
                let value: u8;
                if let Ok(_value) = _param.as_str().parse::<u8>() {
                    value = _value;
                    param = Const(value);
                } else {return None}
            }
            else if let Some(_param) = caps.name("mem") {
                let param_str = _param.as_str();
                let value_str = param_str.substring(1, param_str.len());
                let value: usize;
                if let Ok(_value) = value_str.parse::<usize>() {
                    value = _value;
                } else {return None}
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
                } else {return None}
                param = InstructionIndex(value);
            }
            else {return None};
            instruction = Self::str_to_instruction(&instruction_str, &param);

            
            log!(Self::instruction_to_str(instruction));
        }
        self.store_instructions(instructions);
        return Some(())
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

    fn instruction_to_str(instruction: Instruction) -> String {
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

    }
    
}