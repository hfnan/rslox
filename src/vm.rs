use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::compiler::compile;

macro_rules! binary_op {
    ($self: ident, $op: tt) => {{
        let b = $self.stack.pop().unwrap();
        let a = $self.stack.pop().unwrap();
        $self.stack.push(a $op b);
    }};
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug)]
pub enum InterpretError {
    COMPILE_ERROR,
    RUNTIME_ERROR,
}

type InterpretResult = Result<(), InterpretError>;
struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    fn new(chunk: Chunk) -> Self {
        Self {chunk: chunk, ip: 0, stack: Vec::new()}
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature = "debug_trace_execution")] {
                print!("          ");
                for slot in &self.stack {
                    print!("[ {} ]", slot);
                }
                println!();
                self.chunk.disassemble_instruction(self.ip);
            }
            let instruction = self.read_byte();
            match instruction.into() {
                OpCode::RETURN => {
                    println!("{}", self.stack.pop().unwrap());
                    return Ok(())
                },
                OpCode::CONSTANT => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                },
                OpCode::NEGATE => {
                    let val = self.stack.pop().unwrap();
                    self.stack.push(-val);
                },
                OpCode::ADD => binary_op!(self, +),
                OpCode::SUBTRACT => binary_op!(self, -),
                OpCode::MULTIPLY => binary_op!(self, *),
                OpCode::DIVIDE => binary_op!(self, /),
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let offset = self.read_byte();
        self.chunk.constants[offset as usize]
    }

}

pub fn interpret(source: String) -> InterpretResult {
    compile(source);
    Ok(())
}
