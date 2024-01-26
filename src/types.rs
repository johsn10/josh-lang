#[derive(PartialEq, Clone, Copy)]
pub enum Param {
    Mem(usize),
    Const(i32),
    Index,
    InstructionIndex(usize),
}

#[derive(Clone, Copy)]
pub enum Instruction {
    Add(Param),
    Store(Param),
    Load(Param),
    Inc,
    Dec,
    Jump(Param),
}

