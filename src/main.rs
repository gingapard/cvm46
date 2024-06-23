pub mod error;
pub mod exec;
pub mod utils;
use error::Error;
use exec::*;

const STACK_CAP: usize = 1024;
const MEMORY_CAP: usize = 1024;

#[derive(Debug, Clone, Copy)]
pub enum Word {
    Int(i64),
    Float(f32),
    Double(f64),
    Str(usize),
}

pub struct Machine {
    stack: [Word; STACK_CAP],
    sp: usize,
    memory: [Word; MEMORY_CAP],
    string_memory: Vec<String>,

    ip: usize,
    program: Vec<Inst>,
    halt: bool,
    debug: bool,
}

impl Machine {
    fn new(program: Vec<Inst>) -> Self {
        Machine {
            stack: [Word::Int(0); STACK_CAP],
            sp: 0,
            memory: [Word::Int(0); MEMORY_CAP],
            string_memory: Vec::new(),
            
            ip: 0,
            program,
            halt: false,
            debug: false,
        }
    }
}

fn main() -> Result<(), Error> {
    let program = vec![
        Inst::new(InstType::Pushi, Word::Int(69)),
        Inst::new(InstType::Pushf, Word::Float(2.0)),
        Inst::new(InstType::Mul, Word::Int(0)),
    ];

    let mut machine = Machine::new(program);
    machine.debug = true;
    machine.exec()
}
