use super::*;

pub struct Stack {
    pub stack: Vec<Word>,
    pub sp: usize,
    pub sbp: usize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: Vec::new(),
            sp: 0,
            sbp: 0,
        }
    }

    pub fn push(&mut self, value: Word) -> Result<(), Error> {
        self.stack.push(value);
        self.sp += 1;
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

    // Push arr to stack, Pointing to the first element
    pub fn push_segment(&mut self, arr: &[Word]) -> Result<Pointer, Error> {

        self.push(Word::Int(arr.len() as i64))?;
        let ptr = Pointer::Stack(self.sp);

        for &elem in arr.iter() {
            self.push(elem)?;
        }
        
        Ok(ptr)
    }
}
