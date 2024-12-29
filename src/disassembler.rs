use crate::{ByteCode, OpCode};

pub fn disassembler(bytecode: &ByteCode, filename: &str) {
    println!("== {} ==", filename);

    let mut offset = 0;
    while offset < bytecode.chunk_count() {
        offset = disassemble_instruction(bytecode, offset);
    }
}

pub fn disassemble_instruction(bytecode: &ByteCode, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 {
        let line = match bytecode.get_line(offset) {
            Some(l) => l,
            None => panic!("Invalid line number {}", offset),
        };
        let last_line = match bytecode.get_line(offset - 1) {
            Some(l) => l,
            None => panic!("Invalid line number {}", offset - 1),
        };
        if line == last_line {
            print!("   | ");
        } else {
            print!("{:4} ", line);
        }
    } else {
        match bytecode.get_line(offset) {
            Some(line) => print!("{:4} ", line),
            None => panic!("Invalid line number {}", offset),
        }
    };

    if let Some(chunk) = bytecode.get_chunk(offset) {
        match OpCode::try_from(*chunk) {
            Ok(opcode) => match opcode {
                OpCode::Constant => constant_instruction("CONSTANT", bytecode, offset),
                OpCode::Add => simple_instruction("ADD", offset),
                OpCode::Subtract => simple_instruction("SUBTRACT", offset),
                OpCode::Multiply => simple_instruction("MULTIPLY", offset),
                OpCode::Divide => simple_instruction("DIVIDE", offset),
                OpCode::Negate => simple_instruction("NEGATE", offset),
                OpCode::Return => simple_instruction("RETURN", offset),
            },
            Err(_) => panic!("Unknown opcode {}", chunk),
        }
    } else {
        panic!("Invalid chunk offset {}", offset);
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, bytecode: &ByteCode, offset: usize) -> usize {
    let index = match bytecode.get_chunk(offset + 1) {
        Some(i) => i,
        None => panic!("Invalid chunk offset {}", offset),
    };
    let constant = match bytecode.get_constant(*index) {
        Some(c) => c,
        None => panic!("Invalid constant index {}", index),
    };
    println!("{:16} {} '{}'", name, index, constant);
    offset + 2
}
