use raven_lang::{disassembler, ByteCode, OpCode, VirtualMachine};

fn main() {
    let mut vm = VirtualMachine::new();
    let mut bytecode = ByteCode::new();

    let index = bytecode.push_constant(1.2);
    bytecode.push_chunk(OpCode::Constant.into(), 123);
    bytecode.push_chunk(index, 123);

    let index = bytecode.push_constant(3.4);
    bytecode.push_chunk(OpCode::Constant.into(), 123);
    bytecode.push_chunk(index, 123);

    bytecode.push_chunk(OpCode::Add.into(), 123);

    let index = bytecode.push_constant(5.6);
    bytecode.push_chunk(OpCode::Constant.into(), 123);
    bytecode.push_chunk(index, 123);

    bytecode.push_chunk(OpCode::Divide.into(), 123);
    bytecode.push_chunk(OpCode::Negate.into(), 123);

    bytecode.push_chunk(OpCode::Return.into(), 123);

    disassembler(&bytecode, "test file");

    let result = vm.interpret(&bytecode);
    if let Err(e) = result {
        println!("Error: {:?}", e);
    }
}
