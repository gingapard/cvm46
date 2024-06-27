use super::*;
use crate::error::Error;

#[derive(Debug, Clone)] 
pub enum InstType {
    Pushi,  // Push Integer
    Pushf,  // Push Float (32-bit)
    Pushd,  // Push Double (64-bit)
    Pushc,  // Push Char 
    Pushr,  // Push Register 
    Pushs,  // Push Segment 
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
    Call,   // Call ip
    Return, // Return to ip
    Exit,   // Stop Execution
           
    Cmp,    // Compare
           
    Alloc,  // Allocate Array on Heap
    Free,   // Free Array on Heap
    Set,    // Sets Element. Needs pointer to Target Element

    Mov,    // Copy Value from memory into register
    Loadr,  // Load value from Heap into register
    Storer, // Store register value on heap
 
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
    operand: [Word; 2],
}

impl Inst {
    pub fn new(inst_type: InstType, operand: [Word; 2]) -> Self {
        Inst { inst_type, operand }
    }
}

impl Machine {
    /// Execute whole program
    pub fn exec(&mut self) -> Result<(), Error> {
        while self.ip < self.program.len() && !self.halt {
            if self.exit {
                break;
            }

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
                if let Word::Int(val) = inst.operand[0] {
                    self.push(Word::Int(val))?;
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Pushf => {
                if let Word::Float(val) = inst.operand[0] {
                    self.push(Word::Float(val))?;
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Pushd => {
                if let Word::Double(val) = inst.operand[0] {
                    self.push(Word::Double(val))?;
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Pushc => {
                if let Word::Char(val) = inst.operand[0]{
                    self.push(Word::Char(val))?;
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Pushr => {
                if let Word::Ptr(ptr) = inst.operand[0] {
                    let reg_index = ptr.as_usize();
                    if reg_index >= self.registers.len() {
                        return Err(Error::IllegalInst);
                    }

                    let value = self.registers[reg_index];
                    self.push(value)?;
                }
                else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Pushs => {
                if let Word::Ptr(ptr) = inst.operand[0] {
                    let data_ptr = ptr.as_usize();
                    let segment_ptr = self.push_segment(&self.data[data_ptr].clone())?;
                    let _ = self.push(Word::Ptr(segment_ptr));
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
                if let Word::Int(addr) = inst.operand[0] {
                    if addr < 0 || addr as usize >= self.program.len() {
                        return Err(Error::IllegalJmp);
                    }

                    self.ip = addr as usize;
                } else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Jeq | InstType::Jne => {
                if let Word::Int(addr) = inst.operand[0] {
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
            InstType::Call => { 
                if let Word::Int(addr) = inst.operand[0] {
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
            InstType::Exit => {
                self.exit(inst.operand[0]);
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

            // Allocate space and Push Pointer on Stack
            InstType::Alloc => {
                if let Word::Int(size) = inst.operand[0] {
                    let ptr = self.malloc(size as usize)?;
                    self.push(Word::Ptr(ptr))?;
                }
                else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Free => {
                if let Word::Ptr(ptr) = inst.operand[0] {
                    self.free(ptr)?;
                }
                else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Set => {
                if let Word::Ptr(ptr) = inst.operand[0] {
                    let value = self.pop()?;
                    let _ = self.setelem(ptr, value);
                }
                else {
                    return Err(Error::IllegalInst);
                }
            }
            InstType::Mov => {
                if let Word::Ptr(register_ptr) = inst.operand[0] {

                    let reg_index = register_ptr.as_usize();
                    if reg_index >= self.registers.len() {
                        return Err(Error::IllegalInst);
                    }
                    
                    self.registers[reg_index] = inst.operand[1];
                }
                else {
                    return Err(Error::IllegalInst);
                }
            }

            // Load Single Word from Heap into register
            InstType::Loadr => {
                if let Word::Ptr(ptr) = inst.operand[0] {
                    let reg_index = ptr.as_usize();
                    if reg_index >= self.registers.len() {
                        return Err(Error::IllegalInst);
                    }

                    let heap_ptr = match inst.operand[1] {
                        Word::Ptr(ptr) => ptr.as_usize(),
                        _ => return Err(Error::IllegalInst),
                    };

                    if heap_ptr >= self.heap.len() {
                        return Err(Error::SegmentationFault);
                    }

                    self.registers[reg_index] = self.heap[heap_ptr];
                }
                else {
                    return Err(Error::IllegalInst);
                }
            }

            // Storing single Word Register on Heap change register to now contain pointer
            InstType::Storer => {
                if let Word::Ptr(reg_ptr) = inst.operand[0] {
                    let reg_index = reg_ptr.as_usize();

                    if reg_index >= self.registers.len() {
                        return Err(Error::IllegalInst);
                    }
                    
                    let heap_ptr = self.malloc(1)?;
                    self.setelem(heap_ptr, self.registers[reg_index])?;
                    self.registers[reg_index] = Word::Ptr(heap_ptr);
                } else {
                    return Err(Error::IllegalInst);
                }
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
                self.read()?;
            }
            InstType::Write => {
                if let Word::Ptr(ptr) = inst.operand[0] {
                    self.write(ptr)?;
                }
            }
        }

        Ok(())
    }
}
