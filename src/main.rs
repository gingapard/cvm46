pub mod error;
pub mod exec;
pub mod utils; 
pub mod memory;
pub mod stack;

use error::Error;
use exec::*;
use stack::Stack;

use std::collections::HashMap;
use std::fs::File;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pointer {
    Register(usize),
    Stack(usize),
    Heap(usize),
    Files(usize),
    Data(usize),
}

impl Pointer {
    pub fn as_usize(&self) -> usize {
        let value = match self {
            Pointer::Register(v) => v,
            Pointer::Stack(v) => v,
            Pointer::Heap(v) => v,
            Pointer::Files(v) => v,
            Pointer::Data(v) => v,
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
    None,
}

pub struct Machine {
    registers: [Word; 8],
    data: Vec<Vec<Word>>,

    stack: Stack,

    heap: Vec<Word>,
    hp: usize,

    files: HashMap<usize, File>,
    file_id_counter: usize,

    ip: usize,
    program: Vec<Inst>,
    exit: bool,
    halt: bool,
    debug: bool,
}

impl Machine {
    fn new(program: Vec<Inst>) -> Self {
        Machine {
            registers: [Word::Free; 8],
            data: Vec::new(),

            stack: Stack::new(),

            heap: Vec::new(),
            hp: 0,

            files: HashMap::new(),
            file_id_counter: 0,
            
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
        Inst::new(InstType::Pushs, [Word::Ptr(Pointer::Data(0)), Word::None]),
    ];

    let mut machine = Machine::new(program);
    machine.data.push(vec![
        Word::Char('h'),
        Word::Char('e'),
        Word::Char('l'),
        Word::Char('l'),
        Word::Char('o'),
    ]);

    machine.debug = true;
    machine.exec()?;

    Ok(())
}

