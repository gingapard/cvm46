pub mod error;
pub mod exec;
pub mod utils; use error::Error;
use exec::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Word {
    Int(i64),
    Float(f32),
    Double(f64),
    Ptr(usize),
    Char(char),
}

pub struct Machine {
    stack: Vec<Word>,
    sp: usize,
    sbp: usize,
    heap: Vec<Word>,
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

    let arr = vec![
        Word::Char('H'),
        Word::Char('e'),
        Word::Char('l'),
        Word::Char('l'),
        Word::Char('o'),
    ];

    let mut machine = Machine::new(program);
    machine.debug = true;

    let ptr = machine.push_arr(&arr)?;
    let s = machine.read_arr(&machine.stack, ptr)?;
    println!("{:?}", s);

    Ok(())
}
