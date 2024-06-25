use super::*;
use std::io::Write;
use std::io::stdin;

impl Machine {

    /// Push to Stack
    pub fn push(&mut self, value: Word) -> Result<(), Error> {
        self.stack.push(value);
        self.sp = self.stack.len();
        Ok(())
    }

    /// Pop off stack and return value
    pub fn pop(&mut self) -> Result<Word, Error> {
        if self.sp < 1 {
            return Err(Error::StackUnderflow);
        }

        self.sp -= 1;
        Ok(self.stack.remove(self.sp))
    }

    /// Pushed the top of the stack again, duplicating the value
    pub fn dup(&mut self) -> Result<(), Error> {
        if self.sp < 1 {
            return Err(Error::StackUnderflow);
        }

        let value = self.stack[self.sp - 1].clone();
        let _ = self.push(value)?;
        Ok(())
    }

    /// Enters new Stack Frame
    pub fn enter_frame(&mut self) {
        self.sbp = self.sp;
    }
    
    /// Leave Stack Frame
    pub fn pop_frame(&mut self) {
        self.sp = self.sbp;
    }

    /// Push String to Stack
    pub fn push_string(&mut self, s: &String) -> Result<(), Error> {
        let ptr = self.sp;
        let len = s.len();
        
        self.push(Word::Int(len as i64))?;
        for ch in s.chars() {
            self.push(Word::Char(ch))?;
        }
        
        Ok(())
    }

    pub fn read_string(&self, segment: &Vec<Word>, ptr: usize) -> Result<String, Error> {
        if ptr >= segment.len() {
            return Err(Error::SegmentationFault);
        }
        
        if let Word::Int(len) = segment[ptr] {
            let len = len as usize;
            if ptr + 1 + len > segment.len() {
                return Err(Error::SegmentationFault);
            }
            
            let mut s = String::new();
            for i in ptr + 1..ptr + 1 + len {
                if i >= segment.len() {
                    return Err(Error::SegmentationFault);
                }

                if let Word::Char(c) = segment[i] {
                    s.push(c);
                }
                else {
                    return Err(Error::TypeMismatch);
                }
            }

            Ok(s)
        } 
        else {
            return Err(Error::TypeMismatch);
        }
    }

    /// Open File
    pub fn open(&mut self, filename: &usize) -> Result<(), Error> {
        if self.sp < 1 {
            return Err(Error::StackUnderflow);
        }

        let mode = match self.pop()? {
            Word::Int(mode) => mode,
            _ => return Err(Error::IllegalOperandType),
        };

        // TODO:

        Ok(())
    }

    /// Read from Stdin
    pub fn read(&mut self) -> Result<usize, Error> {
        let mut buffer = String::new();

        stdin().read_line(&mut buffer)
            .map_err(|_| { Error::IO } 
        )?;

        self.push_string(&buffer)?;

        Ok(self.sp) 
    }

    /// Write to stdout
    pub fn write(&self, segment: &Vec<Word>, ptr: usize) -> Result<(), Error> {
        let s = self.read_string(&segment, ptr)?;
        write!(std::io::stdout(), "{}", s)
            .map_err(|_| { Error::IO } 
        )?;

        Ok(())
    }


    pub fn exit(&mut self, exit_code: Word) {
        let _ = self.push(exit_code);
        self.exit = true;
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
        } 
        else {
            for i in 0..self.sp {
                match self.stack[i] {
                    Word::Int(val) => println!("  {} - Int({})", i, val),
                    Word::Float(val) => println!("  {} - Float({})", i, val),
                    Word::Double(val) => println!("  {} - Double({})", i, val),
                    Word::Ptr(val) => println!("  {} -> Pointer({})", i, val),
                    Word::Char(val) => println!("  {} -> Char({})", i, val),
                }
            }
        }
    }
}
