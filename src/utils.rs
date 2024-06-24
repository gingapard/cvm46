use super::*;
use crate::Error::*;
use crate::exec::*;

impl Machine {
    /// Push to Stack
    pub fn push(&mut self, value: Word) -> Result<(), Error> {
        if self.sp >= STACK_CAP {
            return Err(Error::StackOverflow);
        }
        self.stack[self.sp] = value;
        self.sp += 1;
        Ok(())
    }

    /// Pop off stack and return value
    pub fn pop(&mut self) -> Result<Word, Error> {
        if self.sp < 1 {
            return Err(Error::StackUnderflow);
        }
        self.sp -= 1;
        Ok(self.stack[self.sp])
    }

    /// Enters new Stack Frame
    pub fn enter_frame(&mut self) {
        self.sbp = self.sp;
    }
    
    pub fn pop_frame(&mut self) {
        self.sp = self.sbp;
    }

    /// Do Binary Operation based on Word-type 
    /// TODO: add Rc<Word>
    pub fn binary_op<F>(&mut self, op: F) -> Result<(), Error>
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

    pub fn dump(&self) {
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
