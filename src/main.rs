use raven_lang::{disassembler, ByteCode, InstructionSize, OpCode, OPCODE_SHIFT};

fn main() {
    let mut bytecode = ByteCode::new();
    let index = bytecode.push_constant(1.2);
    bytecode.push_instruction(
        ((OpCode::Constant as InstructionSize) << OPCODE_SHIFT) | index,
        123,
    );
    let index = bytecode.push_constant(2 as f64);
    bytecode.push_instruction(
        ((OpCode::Constant as InstructionSize) << OPCODE_SHIFT) | index,
        123,
    );
    bytecode.push_instruction((OpCode::Return as InstructionSize) << OPCODE_SHIFT, 123);
    bytecode.push_instruction((OpCode::Return as InstructionSize) << OPCODE_SHIFT, 124);
    disassembler(&bytecode, "test file");
}
