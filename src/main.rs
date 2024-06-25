pub mod error;
pub mod exec;
pub mod utils;
use error::Error;
use exec::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Word {
    Int(i64),
    Float(f32),
    Double(f64),
    Ptr(usize),
}

pub struct Machine {
    stack: Vec<Word>,
    sp: usize,
    sbp: usize,
    heap: Vec<Word>,
    files: Vec<std::fs::File>,

    ip: usize,
    program: Vec<Inst>,
    halt: bool,
    debug: bool,
}

impl Machine {
    fn new(program: Vec<Inst>) -> Self {
        Machine {
            stack: Vec::new(),
            sp: 0,
            sbp: 0,
            heap: Vec::new(),
            files: Vec::new(),
            
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
        Inst::new(InstType::Pushf, Word::Float(2.3)),
        Inst::new(InstType::Mul, Word::Int(0)),
        Inst::new(InstType::Pushf, Word::Float(2.3)),
        Inst::new(InstType::Mul, Word::Int(0)),
        Inst::new(InstType::Pushf, Word::Float(2.3)),
        Inst::new(InstType::Mul, Word::Int(0)),
        Inst::new(InstType::Pushf, Word::Float(2.3)),
        Inst::new(InstType::Mul, Word::Int(0)),
        Inst::new(InstType::Dup, Word::Int(0)),
    ];

    let mut machine = Machine::new(program);
    machine.debug = true;
    machine.exec()
}
