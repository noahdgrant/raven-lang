use crate::{InterpretError, Scanner, TokenType};

pub fn compile(source: &str) -> Result<(), InterpretError> {
    let mut scanner = Scanner::new(source);
    let mut line: usize = 0;
    loop {
        match scanner.get_token() {
            Ok(token) => {
                if token.line != line {
                    print!("{:4} ", token.line);
                    line = token.line;
                } else {
                    print!("   | ");
                }
                println!("{:?} '{}'", token.typee, token.lexeme);

                if token.typee == TokenType::EOF {
                    return Ok(());
                }
            }
            Err(token) => {
                eprintln!("{}", token.lexeme);
                return Err(InterpretError::CompileError);
            }
        }
    }
}
