pub mod error;
pub mod exec;
pub mod utils; use error::Error;
use exec::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pointer {
    Stack(usize),
    Heap(usize),
    Files(usize),
}

impl Pointer {
    pub fn as_usize(&self) -> usize {
        let value = match self {
            Pointer::Stack(v) => v,
            Pointer::Heap(v) => v,
            Pointer::Files(v) => v,
        };

        return *value;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Word {
    Int(i64),
    Float(f32),
    Double(f64),
    Ptr(Pointer),
    Char(char),
    Free,
}

pub struct Machine {
    stack: Vec<Word>,
    sp: usize,
    sbp: usize,
    heap: Vec<Word>,
    hp: usize,
    files: Vec<std::fs::File>,

    ip: usize,
    program: Vec<Inst>,
    exit: bool,
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
            hp: 0,
            files: Vec::new(),
            
            ip: 0,
            program,

            exit: false,
            halt: false,
            debug: false,
        }
    }
}

fn main() -> Result<(), Error> {
    let program = vec![
    ];

    let mut machine = Machine::new(program);
    machine.debug = true;

    Ok(())
}
