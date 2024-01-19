use std::ops::Index;

use gloo_console::log;
use substring::Substring;
use wasm_bindgen::JsValue;

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
    pub fn parse_str_to_instructions(&mut self, instructions_string: &str) -> Option<()> {
        let instructions: Vec<Instruction> = vec![];

        for instructions_string_line in instructions_string.lines() {
            let instruction: Instruction;
            let param: Param;

            let line_split: Vec<&str> = instructions_string_line.split(" ").collect::<Vec<&str>>();
            if  line_split.len() != 2 || line_split.get(1).unwrap() == &String::new() { return None }
            let line_arg_first_char = line_split.get(1).unwrap().chars().nth(0).unwrap().to_string();

            if line_arg_first_char == "@" {
                let argument_string = line_split[1];
                let num_string = argument_string.substring(1, argument_string.len());
                
                let num: usize;
                if let Ok(num_parsed) = num_string.parse::<usize>() {
                    num = num_parsed;
                } else {return None}
                // Debug
                log!("Memory: ".to_owned() + num_string);
                param = Mem(num);
            }
            else if line_arg_first_char.chars().nth(0).unwrap().is_numeric() {
                let num: u8;
                if let Ok(num_parsed) = line_split[1].parse::<u8>() {
                    num = num_parsed;
                } else {return None}

                // Debug
                log!("Const: ".to_owned() + line_split[1]);
                param = Const(num);
            }
            else if line_split[1] == "index" {
                param = MemIndex;
                // Debug
                log!("Index");
            } else {return None}

            instruction = match line_split[0] {
                "Add" => Add(param),
                "Store" => Store(param),
                "Load" => Load(param),
                "Inc" => Inc,
                "Dec" => Dec,
                "Jump" => Jump(line_split[1].parse::<usize>().unwrap()),
                _ => return None,
            }
            log!(instruction);
        }
        self.store_instructions(instructions);
        Some(())
    }
    pub fn store_instructions(&mut self, instructions: Vec<Instruction>) {

    }
    
}