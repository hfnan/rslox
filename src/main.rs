mod chunk;
mod value;
mod vm;
mod compiler;
mod scanner;

use std::io::Write;

use vm::{interpret, InterpretError};

fn main() {
    let args: Vec<String> = std::env::args().collect(); 

    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage rslox [path]");
            std::process::exit(64);
        }
    }
}

fn repl() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            println!();
            break;
        }

        interpret(line).unwrap();
    }
}

fn run_file(path: &str) {
    let source = std::fs::read_to_string(path).unwrap();
    match interpret(source) {
        Err(InterpretError::COMPILE_ERROR) => std::process::exit(65),
        Err(InterpretError::RUNTIME_ERROR) => std::process::exit(70),
        _ => (),
    }

}
