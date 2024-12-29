use std::fmt;

use crate::{disassemble_instruction, ByteCode, OpCode, Value};

#[derive(Debug, PartialEq, Eq)]
pub enum InterpretError {
    CompileError(String),
    RuntimeError(String),
}

enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct VirtualMachine {
    ip: usize,
    stack: Stack<Value>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: Stack::new(),
        }
    }

    pub fn interpret(&mut self, bytecode: &ByteCode) -> Result<(), InterpretError> {
        while self.ip < bytecode.chunk_count() {
            if cfg!(feature = "debug_trace_execution") {
                println!("          {}", self.stack);
                disassemble_instruction(bytecode, self.ip);
            };

            let chunk = match bytecode.get_chunk(self.ip) {
                Some(c) => c,
                None => panic!("Instruction pointer ({}) > chunk count", self.ip),
            };
            self.ip += 1;
            match OpCode::try_from(*chunk) {
                Ok(opcode) => match opcode {
                    OpCode::Constant => {
                        let index = match bytecode.get_chunk(self.ip) {
                            Some(i) => i,
                            None => panic!("Instruction pointer ({}) > chunk count", self.ip),
                        };
                        self.ip += 1;
                        let constant = match bytecode.get_constant(*index) {
                            Some(c) => c,
                            None => panic!("Invalid constant index {}", chunk),
                        };
                        self.stack.push(*constant);
                    }
                    OpCode::Add => self.binary_op(BinaryOperation::Add),
                    OpCode::Subtract => self.binary_op(BinaryOperation::Subtract),
                    OpCode::Multiply => self.binary_op(BinaryOperation::Multiply),
                    OpCode::Divide => self.binary_op(BinaryOperation::Divide),
                    OpCode::Negate => {
                        if let Some(element) = self.stack.pop() {
                            self.stack.push(-element)
                        }
                    }
                    OpCode::Return => {
                        if let Some(element) = self.stack.pop() {
                            println!("{}", element)
                        }
                    }
                },
                Err(_) => panic!("Unknown opcode {}", chunk),
            }
        }
        Ok(())
    }

    fn binary_op(&mut self, operation: BinaryOperation) {
        let b = match self.stack.pop() {
            Some(element) => element,
            None => panic!("Stack empty"),
        };
        let a = match self.stack.pop() {
            Some(element) => element,
            None => panic!("Stack empty"),
        };

        match operation {
            BinaryOperation::Add => self.stack.push(a + b),
            BinaryOperation::Subtract => self.stack.push(a - b),
            BinaryOperation::Multiply => self.stack.push(a * b),
            BinaryOperation::Divide => self.stack.push(a / b),
        };
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

struct Stack<T> {
    stack: Vec<T>,
}

// https://www.kirillvasiltsov.com/writing/how-to-write-a-stack-in-rust/
impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }
}

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.stack {
            write!(f, "[ {} ]", item)?;
        }
        Ok(())
    }
}
