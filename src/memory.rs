use gloo_console::log;
use substring::Substring;
use wasm_bindgen::JsValue;
use regex::Regex;

use crate::interpreter::Instruction::{*, self};
use crate::interpreter::InstructionIndex::{*, self};
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
        let re = Regex::new(r"((?<add>Add)|(?<store>Store)|(?<load>Load)) ((?<const>\d+)|(?<memi>@\d+)|(?<index>@Index))|(?<inc>Inc)|(?<dec>Dec)|(?<jump>Jump) (?<instruction_index>#\d+)").unwrap();
        let mut instructions: Vec<Instruction> = vec![];
        for instruction_line in instructions_str.lines() {
            let mut instruction: Instruction = NoInstruction;

            let Some(caps) = re.captures(instruction_line) else {
                return None;
            };
            let instructions_with_params_str = ["add", "store", "load"];
            let params_str = ["const", "memi", "index"];

            for current_instruction_str in instructions_with_params_str {
                if caps.name(current_instruction_str).is_some() {
                    for current_param in params_str {
                        if caps.name(current_param).is_some() {
                            let param: Param;
                            if let Some(_param) = caps.name(current_param) {
                                param = Self::match_str_to_param(_param.as_str()).unwrap();
                            } else {return None}
                            instruction = Self::match_str_to_instruction(caps.name(current_instruction_str).unwrap().as_str(), param).unwrap();
                            return Some(());
                        }
                    }
                    return None;
                }
            }
            if caps.name("Jump").is_some() {
                if caps.name("instructions_index").is_some() {
                    instruction = Jump(NoIndex);
                    log!(Self::instruction_to_str(instruction));
                    return Some(());
                }
                return None;
            }
            instructions.push(instruction);
        }
        self.store_instructions(instructions);
        return Some(())
    }

    pub fn match_str_to_instruction(instruction_str: &str, param: Param) -> Option<Instruction> {
        let instruction: Instruction = match instruction_str {
            "Add" => Add(param),
            "Store" => Store(param),
            "Load" => Load(param),
            "Inc" => Inc,
            "Dec" => Dec,
            "Jump" => Jump(NoIndex),
            _ => return None,
        };
        Some(instruction)
    }
    pub fn match_str_to_param(param_str: &str) -> Option<Param> {
        let param: Param = match param_str {
            "const" => Const(0),
            "memi" => Mem(0),
            "index" => MemIndex,
            _ => return None,
        };
        Some(param)
    }
    fn instruction_to_str(instruction: Instruction) -> String {
        let instruction_str: &str = match instruction {
            Add(_) => "Add",
            Store(_) => "Store",
            Load(_) => "Load",
            Inc => "Inc",
            Dec => "Dec",
            Jump(_) => "Jump",
            NoInstruction => "No instruction",
        };
        instruction_str.to_owned()
    }
    pub fn store_instructions(&mut self, instructions: Vec<Instruction>) {

    }
    
}