use super::*;
use std::io::Write;
use std::io::stdin;

impl Machine {

    /// Push to Stack
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

    /// Free heap-allocated segment
    pub fn free(&mut self, ptr: Pointer) -> Result<(), Error> {
        let segment = match ptr {
            Pointer::Stack(_) => &mut self.stack,
            Pointer::Heap(_) => &mut self.heap,
            _ => return Err(Error::InvalidPointer),
        };

        let ptr = ptr.as_usize();
        if ptr >= segment.len() {
            return Err(Error::SegmentationFault);
        }

        if let Word::Int(len) = segment[ptr] {
            let len = len as usize;
            let end = ptr + len;
            let start = ptr + 1;

            // Checking if the slice pointers are valid
            if end > segment.len() || start > segment.len() || start >= end {
                return Err(Error::SegmentationFault);
            }

            // Freeing the segment by setting all elements to Word::Free
            for i in start..end {
                segment[i] = Word::Free;
            }

            segment[ptr] = Word::Free;
        } 
        else {
            return Err(Error::TypeMismatch);
        }

        Ok(())
    }


    // Allocates Word's on the Heap
    pub fn malloc(&mut self, len: usize) -> Result<Pointer, Error> {
        let mut start_index = None;
        let mut segment_length = 0;

        for (index, word) in self.heap.iter().enumerate() {
            match word {
                Word::Free => {
                    if segment_length == 0 {
                        start_index = Some(index);
                    }
                    segment_length += 1;

                    // +1 to fit the length
                    if segment_length >= len + 1 {
                        break;
                    }
                }
                _ => {
                    start_index = None;
                    segment_length = 0;
                }
            }
        }

        // Returns Pointer to suitable segment
        if let Some(start) = start_index {
            self.heap[start] = Word::Int(len as i64);
            return Ok(Pointer::Heap(start + 1)); 
        }

        // Expands heap if no suitable segments already
        let start_index = self.heap.len();
        self.heap.push(Word::Int(len as i64));
        for _ in 0..len {
            self.heap.push(Word::Int(0));
            self.hp += 1;
        }

        Ok(Pointer::Heap(start_index + 1)) 
    }

    /// Sets Element 
    pub fn setelem(&mut self, elem: Pointer, value: Word) -> Result<(), Error> {
        let segment = match elem {
            Pointer::Heap(_) => &mut self.heap,
            Pointer::Stack(_) => &mut self.stack,
            _ => return Err(Error::InvalidPointer),
        };

        let elem_ptr = elem.as_usize();

        // Check that pointer is within bounds
        if elem_ptr >= segment.len() || segment[elem_ptr] == Word::Free {
            return Err(Error::SegmentationFault);
        }
        
        segment[elem_ptr] = value;

        Ok(())
    }

    /// Reads String from Memory
    pub fn read_arr(&self, ptr: Pointer) -> Result<Vec<Word>, Error> {
        let segment = match ptr {
            Pointer::Heap(_) => &self.heap,
            Pointer::Stack(_) => &self.stack,
            _ => return Err(Error::InvalidPointer),
        };

        let ptr = ptr.as_usize();
        let len_ptr = ptr - 1;

        if ptr >= segment.len() {
            return Err(Error::SegmentationFault);
        }
        
        if len_ptr >= segment.len() {
            return Err(Error::SegmentationFault);
        }
        
        if let Word::Int(len) = segment[len_ptr] {
            let len = len as usize;

            let start = ptr;
            let end = start + len;
            let arr_slice = &segment[start..end];
            let arr = arr_slice.to_vec();
            
            Ok(arr)
        } 
        else {
            return Err(Error::InvalidPointer);
        }
    }

    /// Read from Stdin returns pointer
    pub fn read(&mut self) -> Result<usize, Error> {
        let mut buffer = String::new();

        stdin().read_line(&mut buffer)
            .map_err(|_| { Error::IO } 
        )?;
        
        if buffer.ends_with('\n') {
            buffer.pop();
        }

        let str_arr: Vec<Word> = buffer.chars().map(Word::Char).collect();
        let _ = self.push_segment(&str_arr);

        Ok(self.sp) 
    }

    /// Write to stdout from string ptr
    pub fn write(&self, ptr: Pointer) -> Result<(), Error> {
        let arr = self.read_arr(ptr)?;

        // Convert to String
        let string: String = arr.iter()
            .map(|word| Ok(match word {
                Word::Char(c) => *c,
                _ => return Err(Error::TypeMismatch),
            }))
            .collect::<Result<String, Error>>()?;

        // Write the string to stdout
        write!(std::io::stdout(), "{}", string)
            .map_err(|_| Error::IO)?;

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

        println!("\nStack:");
        if self.sp < 1 {
            println!("  [empty]");
        } 
        else {
            for i in 0..self.sp {
                match self.stack[i] {
                    Word::Int(val) => println!("  {} - Int({})", i, val),
                    Word::Float(val) => println!("  {} - Float({})", i, val),
                    Word::Double(val) => println!("  {} - Double({})", i, val),
                    Word::Ptr(val) => println!("  {} -> Pointer({})", i, val.as_usize()),
                    Word::Char(val) => println!("  {} -> Char({})", i, val),
                    _ => continue,
                }
            }
        }
    }
}
