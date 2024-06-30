use super::*;
use std::io::Write;
use std::io::stdin;
use std::fs::OpenOptions;

impl Machine {

    // Open file and return Pointer::Files 
    pub fn open(&mut self, data_ptr: Pointer, mode: i64) -> Result<Pointer, Error> {
        let arr = self.read_arr(data_ptr)?;
        
        let filename: String = arr.iter()
            .map(|word| Ok(match word {
                Word::Char(c) => *c,
                _ => return Err(Error::TypeMismatch),
            }))
            .collect::<Result<String, Error>>()?;

        let file_ptr = match mode {
            0 => OpenOptions::new().read(true).open(filename),
            1 => OpenOptions::new().write(true).create(true).truncate(true).open(filename),
            2 => OpenOptions::new().write(true).create(true).append(true).open(filename),
            _ => return Err(Error::IllegalInst)
        };

        match file_ptr {
            Ok(f) => {
                let file_id = self.file_id_counter;
                self.files.insert(file_id, f);
                self.file_id_counter += 1;
                Ok(Pointer::Files(file_id))
            }
            Err(_) => Err(Error::FileNotFound),
        }
    }

    /// Close Open Files
    pub fn close(&mut self, ptr: Pointer) -> Result<(), Error> {
        if let Pointer::Files(file_ptr) = ptr {
            if let Some(file) = self.files.remove(&file_ptr) {
                return Ok(()); 
            }
            
        }
        else {
            return Err(Error::FileNotFound);
        }
        
        Ok(())
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
        let _ = self.stack.push_segment(&str_arr);

        Ok(self.stack.sp) 
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
        let _ = self.stack.push(exit_code);
        self.exit = true;
    }

    /// Do Binary Operation based on Word-type 
    /// TODO: add Rc<Word>
    pub fn binary_op<F>(&mut self, op: F) -> Result<(), Error>
    where
        F: Fn(Word, Word) -> Result<Word, Error>,
    {
            let right = self.stack.pop()?;
            let left = self.stack.pop()?;
            
            match (&left, &right) {
                (Word::Int(_), Word::Int(_)) => op(left.clone(), right.clone()).and_then(|res| self.stack.push(res)),
                (Word::Float(_), Word::Float(_)) => op(left.clone(), right.clone()).and_then(|res| self.stack.push(res)),
                (Word::Double(_), Word::Double(_)) => op(left.clone(), right.clone()).and_then(|res| self.stack.push(res)),
                (Word::Int(a), Word::Float(_)) => {
                    let a_float = *a as f32; 
                    let result = op(Word::Float(a_float), right.clone())?;
                    self.stack.push(result)
                }
                (Word::Float(_), Word::Int(b)) => {
                    let b_float = *b as f32;
                    let result = op(left.clone(), Word::Float(b_float))?;
                    self.stack.push(result)
                }
                (Word::Float(a), Word::Double(_)) => {
                    let a_double = *a as f64; 
                    let result = op(Word::Double(a_double), right.clone())?;
                    self.stack.push(result)
                }
                (Word::Double(_), Word::Float(b)) => {
                    let b_double = *b as f64;
                    let result = op(left.clone(), Word::Double(b_double))?;
                    self.stack.push(result)
                }
                _ => Err(Error::IllegalInst), 
            }
    }


    pub fn dump(&self) {

        println!("\nStack:");
        if self.stack.sp < 1 {
            println!("  [empty]");
        } 
        else {
            for i in 0..self.stack.sp {
                match self.stack.stack[i] {
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
