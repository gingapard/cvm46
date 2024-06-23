pub mod error;
use error::Error;

const STACK_CAP: usize = 1024;
const MEMORY_CAP: usize = 1024;

#[derive(Debug, Clone, Copy)]
enum Word {
    Int(i64),
    Float(f32),
    Double(f64),
    Str(usize),
}

#[derive(Debug, Clone)]
enum InstType {
    Pushi,
    Pushf,
    Pushd,
    Pushs,
    Pop,
    Plus,
    Sub,
    Mul,
    Div,
    Jmp,
    Cmp,
    Store,
    Load,
    Halt,
}

#[derive(Debug, Clone)]
struct Inst {
    inst_type: InstType,
    operand: Word,
}

impl Inst {
    fn new(inst_type: InstType, operand: Word) -> Self {
        Inst { inst_type, operand }
    }
}

struct Machine {
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

    /// Execute whole program
    fn exec(&mut self) -> Result<(), Error> {
        while self.ip < self.program.len() && !self.halt {
            let inst = self.program[self.ip].clone();
            self.ip += 1;
            self.exec_inst(&inst)?;
            if self.debug {
                self.dump();
            }
        }
        Ok(())
    }

    fn exec_inst(&mut self, inst: &Inst) -> Result<(), Error> {
        match inst.inst_type {
            InstType::Pushi => {
                if let Word::Int(val) = inst.operand {
                    self.push(Word::Int(val))?;
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Pushf => {
                if let Word::Float(val) = inst.operand {
                    self.push(Word::Float(val))?;
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Pushd => {
                if let Word::Double(val) = inst.operand {
                    self.push(Word::Double(val))?;
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Pushs => {
                if let Word::Str(index) = inst.operand {
                    if index < self.string_memory.len() {
                        self.push(Word::Str(index))?;
                    } else {
                        return Err(Error::IllegalInst);
                    }
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Pop => {
                self.pop()?;
            }
            InstType::Plus => {
                if self.sp < 2 {
                    return Err(Error::StackUnderflow);
                }

                self.binary_op(|a, b| match (a, b) {
                    (Word::Int(a), Word::Int(b)) => Ok(Word::Int(a + b)),
                    (Word::Float(a), Word::Float(b)) => Ok(Word::Float(a + b)),
                    (Word::Double(a), Word::Double(b)) => Ok(Word::Double(a + b)),
                    _ => Err(Error::IllegalInst),
                })?;
            }
            InstType::Sub => {
                if self.sp < 2 {
                    return Err(Error::StackUnderflow);
                }

                self.binary_op(|a, b| match (a, b) {
                    (Word::Int(a), Word::Int(b)) => Ok(Word::Int(a - b)),
                    (Word::Float(a), Word::Float(b)) => Ok(Word::Float(a - b)),
                    (Word::Double(a), Word::Double(b)) => Ok(Word::Double(a - b)),
                    _ => Err(Error::IllegalInst),
                })?;
            }
            InstType::Mul => {
                if self.sp < 2 {
                    return Err(Error::StackUnderflow);
                }

                self.binary_op(|a, b| match (a, b) {
                    (Word::Int(a), Word::Int(b)) => Ok(Word::Int(a * b)),
                    (Word::Float(a), Word::Float(b)) => Ok(Word::Float(a * b)),
                    (Word::Double(a), Word::Double(b)) => Ok(Word::Double(a * b)),
                    _ => Err(Error::IllegalInst),
                })?;
            }
            InstType::Div => {
                if self.sp < 2 {
                    return Err(Error::StackUnderflow);
                }

                self.binary_op(|a, b| match (a, b) {
                    (Word::Int(a), Word::Int(b)) => {
                        if b == 0 {
                            Err(Error::DivByZero)
                        } else {
                            Ok(Word::Int(a / b))
                        }
                    }
                    (Word::Float(a), Word::Float(b)) => {
                        if b == 0.0 {
                            Err(Error::DivByZero)
                        } else {
                            Ok(Word::Float(a / b))
                        }
                    }
                    (Word::Double(a), Word::Double(b)) => {
                        if b == 0.0 {
                            Err(Error::DivByZero)
                        } else {
                            Ok(Word::Double(a / b))
                        }
                    }
                    _ => Err(Error::IllegalInst),
                })?;
            }
            InstType::Jmp => {
                if let Word::Int(addr) = inst.operand {
                    if addr < 0 || addr as usize >= self.program.len() {
                        return Err(Error::IllegalJmp);
                    }

                    self.ip = addr as usize;
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Cmp => {
                if self.sp < 2 {
                    return Err(Error::StackUnderflow);
                }
                self.binary_op(|a, b| match (a, b) {
                    (Word::Int(a), Word::Int(b)) => Ok(Word::Int((a == b) as i64)),
                    (Word::Float(a), Word::Float(b)) => Ok(Word::Int((a == b) as i64)),
                    (Word::Double(a), Word::Double(b)) => Ok(Word::Int((a == b) as i64)),
                    (Word::Str(_), Word::Str(_)) => Err(Error::IllegalInst), 
                    _ => Err(Error::IllegalInst),
                })?;
            }
            InstType::Store => {
                if let Word::Int(addr) = inst.operand {
                    if addr < 0 || addr as usize >= MEMORY_CAP {
                        return Err(Error::SegmentationFault);
                    }

                    if self.sp < 1 {
                        return Err(Error::StackUnderflow);
                    }

                    let value = self.pop()?;
                    self.memory[addr as usize] = value;
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Load => {
                if let Word::Int(addr) = inst.operand {
                    if addr < 0 || addr as usize >= MEMORY_CAP {
                        return Err(Error::SegmentationFault);
                    }
                    let value = self.memory[addr as usize];
                    self.push(value)?;
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Halt => {
                self.halt = true;
            }
        }

        Ok(())
    }

    /// Push to Stack
    fn push(&mut self, value: Word) -> Result<(), Error> {
        if self.sp >= STACK_CAP {
            return Err(Error::StackOverflow);
        }
        self.stack[self.sp] = value;
        self.sp += 1;
        Ok(())
    }

    /// Pop off stack and return value
    fn pop(&mut self) -> Result<Word, Error> {
        if self.sp < 1 {
            return Err(Error::StackUnderflow);
        }
        self.sp -= 1;
        Ok(self.stack[self.sp])
    }

    /// Do Binary Operation based on Word-type 
    /// TODO: add Rc<Word>
    fn binary_op<F>(&mut self, op: F) -> Result<(), Error>
        where
            F: Fn(Word, Word) -> Result<Word, Error>,
        {
            let right = self.pop()?;
            let left = self.pop()?;
            
            match (&left, &right) {
                (Word::Int(_), Word::Int(_)) => op(left.clone(), right.clone()).and_then(|res| self.push(res)),
                (Word::Float(_), Word::Float(_)) => op(left.clone(), right.clone()).and_then(|res| self.push(res)),
                (Word::Double(_), Word::Double(_)) => op(left.clone(), right.clone()).and_then(|res| self.push(res)),
                (Word::Int(a), Word::Float(_)) => {
                    let a_float = *a as f32; 
                    let result = op(Word::Float(a_float), right.clone())?;
                    self.push(result)
                }
                (Word::Float(_), Word::Int(b)) => {
                    let b_float = *b as f32;
                    let result = op(left.clone(), Word::Float(b_float))?;
                    self.push(result)
                }
                (Word::Float(a), Word::Double(_)) => {
                    let a_double = *a as f64; 
                    let result = op(Word::Double(a_double), right.clone())?;
                    self.push(result)
                }
                (Word::Double(_), Word::Float(b)) => {
                    let b_double = *b as f64;
                    let result = op(left.clone(), Word::Double(b_double))?;
                    self.push(result)
                }
                _ => Err(Error::IllegalInst), 
            }
        }


    fn dump(&self) {
        println!("Stack:");
        if self.sp < 1 {
            println!("  [empty]");
        } else {
            for i in 0..self.sp {
                match self.stack[i] {
                    Word::Int(val) => println!("  {} - Int({})", i, val),
                    Word::Float(val) => println!("  {} - Float({})", i, val),
                    Word::Double(val) => println!("  {} - Double({})", i, val),
                    Word::Str(index) => println!("  {} - Str({})", i, self.string_memory[index]),
                }
            }
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
