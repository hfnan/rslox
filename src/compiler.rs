use crate::scanner::{Scanner, TOKEN};

pub fn compile(source: String) {
    let mut scanner = Scanner::new(source);

    let mut line = 0;
    loop {
        let token = scanner.scan_token().unwrap();
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }
        println!("{:2?} '{}'", token.ty, &token.source[token.start..(token.start + token.length)]);
    
        if let TOKEN::EOF = token.ty {
            break;
        }
    }    
}