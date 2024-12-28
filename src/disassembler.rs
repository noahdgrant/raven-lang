use crate::{ByteCode, OpCode, OPCODE_MASK, OPCODE_SHIFT, OPERAND_MASK};

pub fn disassembler(bytecode: &ByteCode, filename: &str) {
    println!("== {} ==", filename);

    for (offset, instruction) in bytecode.get_instructions().iter().enumerate() {
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
                print!("{:04} ", line);
            }
        } else {
            match bytecode.get_line(offset) {
                Some(line) => print!("{:04} ", line),
                None => panic!("Invalid line number {}", offset),
            }
        }

        let opcode = (instruction >> OPCODE_SHIFT) & OPCODE_MASK;
        let operands = instruction & OPERAND_MASK;
        match OpCode::try_from(opcode) {
            Ok(op) => match op {
                OpCode::Return => println!("RETURN"),
                OpCode::Constant => constant_instruction(bytecode, operands),
            },
            Err(_) => panic!("Unknown opcode {}", opcode),
        }
    }
}

fn constant_instruction(bytecode: &ByteCode, operands: u64) {
    let index = operands as usize;
    match bytecode.get_constant(index) {
        Some(constant) => println!("CONSTANT {:16} {}", index, constant),
        None => panic!("No constant at index {}", index),
    }
}
