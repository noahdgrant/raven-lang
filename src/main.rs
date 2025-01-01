use std::io::Write;
use std::{env, fs, io, process::exit};

use raven_lang::{InterpretError, VirtualMachine};

fn repl() {
    let mut vm = VirtualMachine::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if let Err(e) = vm.interpret(&line) {
            eprintln!("{:?}", e);
            io::stderr().flush().unwrap();
        }
    }
}

fn run_file(path: &str) {
    let mut vm = VirtualMachine::new();

    let source = fs::read_to_string(path).expect("Should have been able to read the file");
    if let Err(e) = vm.interpret(&source) {
        match e {
            InterpretError::CompileError => exit(65),
            InterpretError::RuntimeError => exit(70),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: raven [path]");
        exit(64);
    }
}
