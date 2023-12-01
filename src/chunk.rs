use crate::value::Value;

// implement From<u8> for specified enum 
// which convert an u8 value to an enum value
macro_rules! from_u8 {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl From<u8> for $name {
            fn from(v: u8) -> Self {
                match v {
                    $(x if x == $name::$vname as u8 => $name::$vname,)*
                    _ => panic!("Invalid u8 value for OpCode"),
                }
            }
        }
    }
}

from_u8! {
    #[repr(u8)]
    pub enum OpCode {
        RETURN, CONSTANT, NEGATE, ADD, SUBTRACT, MULTIPLY, DIVIDE,
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    lines: Vec<usize>,
}


impl Chunk {
    pub fn new() -> Self {
        Self {code: Vec::new(), constants: Vec::new(), lines: Vec::new()}
    }

    pub fn write_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

impl Chunk {
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {name} ==");

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }
        
        let instruction = self.code[offset];
        match instruction.into() {
            OpCode::RETURN => self.simple_instruction("OP_RETURN", offset),
            OpCode::CONSTANT => self.constant_instruction("OP_CONSTANT", offset),
            OpCode::NEGATE => self.simple_instruction("OP_NEGATE", offset),
            OpCode::ADD => self.simple_instruction("OP_ADD", offset),
            OpCode::SUBTRACT => self.simple_instruction("OP_SUBTRACT", offset),
            OpCode::MULTIPLY => self.simple_instruction("OP_MULTIPLY", offset),
            OpCode::DIVIDE => self.simple_instruction("OP_DIVIDE", offset),
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        println!("{:-16} {:4} '{}'", name, constant, self.constants[constant as usize]);
        offset + 2
    }
}