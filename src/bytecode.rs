use std::ops::Index;

use crate::{ConstantPool, Value};

pub type InstructionSize = u64;
pub const OPCODE_SHIFT: u64 = 56;
pub const OPCODE_MASK: u64 = 0xFF;
pub const OPERAND_MASK: u64 = 0xFFFFFFFFFFFFFF;

/// The first byte is reserved for the OpCode that the rest is for operands
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum OpCode {
    /// Constant: operand is constant index
    Constant = 0,
    /// Return: no operands
    Return,
}

impl From<OpCode> for InstructionSize {
    fn from(op: OpCode) -> Self {
        op as InstructionSize
    }
}

impl TryFrom<InstructionSize> for OpCode {
    type Error = ();
    fn try_from(value: InstructionSize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Constant),
            1 => Ok(OpCode::Return),
            _ => Err(()),
        }
    }
}

pub struct ByteCode {
    instructions: Vec<InstructionSize>,
    constants: ConstantPool,
    lines: Vec<u64>,
}

impl ByteCode {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: ConstantPool::new(),
            lines: Vec::new(),
        }
    }

    pub fn push_instruction(&mut self, instruction: InstructionSize, line: u64) {
        self.instructions.push(instruction);
        self.lines.push(line);
    }

    pub fn get_instructions(&self) -> &Vec<InstructionSize> {
        &self.instructions
    }

    pub fn push_constant(&mut self, constant: Value) -> u64 {
        self.constants.push_value(constant)
    }

    pub fn get_constant(&self, index: usize) -> Option<&Value> {
        self.constants.get(index)
    }

    pub fn get_line(&self, index: usize) -> Option<&u64> {
        self.lines.get(index)
    }
}

impl Index<usize> for ByteCode {
    type Output = u64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.lines[index]
    }
}
