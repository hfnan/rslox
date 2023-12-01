use crate::scanner::{Scanner, TOKEN};

pub fn compile(source: String) {
    let mut scanner = Scanner::new(source);

    let mut line = 0;
    loop {
        let token = scanner.scan_token().unwrap();
        if token.line != line {
            println!("{:4} ", token.line);
            line = token.line;
        } else {
            println!("    | ");
        }
        println!("{:2?} '{} {}'", token.ty, token.length, token.start);
    
        if let TOKEN::EOF = token.ty {
            break;
        }
    }    
}