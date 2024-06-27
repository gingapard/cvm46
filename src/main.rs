pub mod error;
pub mod exec;
pub mod utils; use error::Error;
use exec::*;

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
    stack: Vec<Word>,
    sp: usize,
    sbp: usize,
    heap: Vec<Word>,
    hp: usize,
    files: Vec<std::fs::File>,
    data: Vec<Vec<Word>>,

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
            stack: Vec::new(),
            sp: 0,
            sbp: 0,
            heap: Vec::new(),
            hp: 0,
            files: Vec::new(),
            data: Vec::new(),
            
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

