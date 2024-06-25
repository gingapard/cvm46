use super::*;
use crate::error::Error;

#[derive(Debug, Clone)]
pub enum InstType {

    Pushi,  // Push Integer
    Pushf,  // Push Float (32-bit)
    Pushd,  // Push Double (64-bit)
    Pop,    // Pop Stack
    Dup,    // Duplicate
          
    Plus,   // Plus op
    Sub,    // Sub op
    Mul,    // Mul op
    Div,    // Div op
           
    And,    // Bitwise And op
    Or,     // Bitwise Or op
    Xor,    // Bitwise Xor op
    Not,    // Bitwise Not op
            
    Jmp,    // Jump
    Jeq,    // Jump if Equal
    Jne,    // Jump if not Equal
    Halt,   // Halt Execution 
    Exit,   // Stop Execution
           
    Cmp,    // Compare
           
    Store,  // Store on Heap
    Load,   // Load from Heap
            
    Call,   // Call ip
    Return, // Return to ip

    Open,   // Open File
    Close,  // Close File
    Readf,  // Read File
    Writef, // Write File

    Read,   // Read Stdin
    Write,  // Write Stdout
}

#[derive(Debug, Clone)]
pub struct Inst {
    inst_type: InstType,
    operand: Word,
}
impl Inst {
    pub fn new(inst_type: InstType, operand: Word) -> Self {
        Inst { inst_type, operand }
    }
}

impl Machine {
    /// Execute whole program
    pub fn exec(&mut self) -> Result<(), Error> {
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
            InstType::Pop => {
                self.pop()?;
            }
            InstType::Dup => {
                self.dup()?;
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
            InstType::And => {
                self.binary_op(|a, b| match (a, b) {
                    (Word::Int(a), Word::Int(b)) => Ok(Word::Int(a & b)),
                    _ => Err(Error::IllegalInst),
                })?;
            }
            InstType::Or => {
                self.binary_op(|a, b| match (a, b) {
                    (Word::Int(a), Word::Int(b)) => Ok(Word::Int(a | b)),
                    _ => Err(Error::IllegalInst),
                })?;
            }
            InstType::Xor => {
                self.binary_op(|a, b| match (a, b) {
                    (Word::Int(a), Word::Int(b)) => Ok(Word::Int(a ^ b)),
                    _ => Err(Error::IllegalInst),
                })?;
            }
            InstType::Not => {
                if self.sp < 1 {
                    return Err(Error::StackUnderflow);
                }

                let value = self.pop()?;
                match value {
                    Word::Int(a) => self.push(Word::Int(!a))?,
                    _ => return Err(Error::IllegalInst),
                }
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
            InstType::Jeq | InstType::Jne => {
                if let Word::Int(addr) = inst.operand {
                    if addr < 0 || addr as usize >= self.program.len() {
                        return Err(Error::IllegalJmp);

                    }

                    if self.sp < 1 {
                        return Err(Error::StackUnderflow);
                    }
                    
                    let value = self.pop()?;
                    match inst.inst_type {
                        InstType::Jeq => {
                            if value == Word::Int(1 /* true */ ) {
                                self.ip = addr as usize;
                            }
                        }
                        InstType::Jne => {
                            if value == Word::Int(0 /* false */ ) {
                                self.ip = addr as usize;
                            }
                        }
                        _ => unreachable!(),
                    }
                } else {
                    return Err(Error::IllegalJmp);
                };
            }
            InstType::Halt => {
                self.halt = true;
            }
            InstType::Exit => {
                if let Word::Int(exit_code) = inst.operand {
                    std::process::exit(exit_code as i32);
                }
                else {
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
                    _ => Err(Error::IllegalInst),
                })?;
            }
            InstType::Store => {
                if let Word::Int(addr) = inst.operand {
                    if addr < 0 || addr as usize > (self.heap.len() - 1) {
                        return Err(Error::SegmentationFault);
                    }

                    if self.sp < 1 {
                        return Err(Error::StackUnderflow);
                    }

                    let value = self.pop()?;
                    self.heap[addr as usize] = value;
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Load => {
                if let Word::Int(addr) = inst.operand {
                    if addr < 0 || addr as usize > (self.heap.len() - 1) {
                        return Err(Error::SegmentationFault);
                    }

                    let value = self.heap[addr as usize];
                    self.push(value)?;
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Call => { 
                if let Word::Int(addr) = inst.operand {
                    self.push(Word::Int(self.sbp as i64))?;
                    self.sbp = self.sp;

                    self.push(Word::Int(self.ip as i64))?;
                    self.ip = addr as usize;
                }

            }
            InstType::Return => {
                self.ip = match self.pop()? {
                    Word::Int(addr) => addr as usize,
                    _ => return Err(Error::IllegalInst),
                };

                self.sbp = match self.pop()? {
                    Word::Int(sbp) => sbp as usize,
                    _ => return Err(Error::IllegalInst),
                };

            }
            InstType::Open => {
                // TODO
            }
            InstType::Close => {
                // TODO
            }
            InstType::Readf => {
                // TODO
            }
            InstType::Writef => {
                // TODO
            }
            InstType::Read => {
                // TODO
            }
            InstType::Write => {
                // TODO
            }
        }

        Ok(())
    }
}
