use std::ops::Index;

use crate::{ConstantPool, Value};

pub type Chunk = usize;

/// The first byte is reserved for the OpCode that the rest is for operands
#[repr(usize)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum OpCode {
    Constant = 0,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate = 5,
    Return = 6,
}

impl From<OpCode> for Chunk {
    fn from(op: OpCode) -> Self {
        op as Chunk
    }
}

impl TryFrom<Chunk> for OpCode {
    type Error = ();
    fn try_from(value: Chunk) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Constant),
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Subtract),
            3 => Ok(OpCode::Multiply),
            4 => Ok(OpCode::Divide),
            5 => Ok(OpCode::Negate),
            6 => Ok(OpCode::Return),
            _ => Err(()),
        }
    }
}

pub struct ByteCode {
    chunks: Vec<Chunk>,
    constants: ConstantPool,
    lines: Vec<usize>,
}

impl ByteCode {
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            constants: ConstantPool::new(),
            lines: Vec::new(),
        }
    }

    pub fn push_chunk(&mut self, chunk: Chunk, line: usize) {
        self.chunks.push(chunk);
        self.lines.push(line);
    }

    pub fn get_chunk(&self, index: usize) -> Option<&Chunk> {
        self.chunks.get(index)
    }

    pub fn get_chunks(&self) -> &Vec<Chunk> {
        &self.chunks
    }

    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    pub fn push_constant(&mut self, constant: Value) -> usize {
        self.constants.push_value(constant)
    }

    pub fn get_constant(&self, index: usize) -> Option<&Value> {
        self.constants.get(index)
    }

    pub fn get_line(&self, index: usize) -> Option<&usize> {
        self.lines.get(index)
    }
}

impl Default for ByteCode {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for ByteCode {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.lines[index]
    }
}
