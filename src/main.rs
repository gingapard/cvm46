pub mod error;
use error::Error;

type Word = i64;
const STACK_CAP: usize = 1024;
const MEMORY_CAP: usize = 1024;

#[derive(Debug, Clone)]
enum InstType {
    Push,
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

    ip: usize,
    program: Vec<Inst>,
    halt: bool,

    // Makes it dump the stack on instruction execs
    debug: bool
}

impl Machine {
    fn new(program: Vec<Inst>) -> Self {
        Machine {
            stack: [0; STACK_CAP],
            sp: 0,
            memory: [0; MEMORY_CAP],

            ip: 0,
            program,
            halt: false,

            debug: false
        }
    }

    /// Increments the ip and executes next instruction
    fn exec(&mut self) -> Result<(), Error> {
        while self.ip < self.program.len() && self.halt == false { 
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
            InstType::Push => {
                if self.sp >= STACK_CAP {
                    return Err(Error::StackOverflow);
                }
                self.stack[self.sp] = inst.operand;
                self.sp += 1;
            }
            InstType::Pop => {
                if self.sp < 1 {
                    return Err(Error::StackUnderflow);
                }
                self.sp -= 1;
            }
            InstType::Plus => {
                if self.sp < 2 {
                    return Err(Error::StackUnderflow);
                }
                self.stack[self.sp - 2] += self.stack[self.sp - 1];
                self.sp -= 1;
            }
            InstType::Sub => {
                if self.sp < 2 {
                    return Err(Error::StackUnderflow);
                }
                self.stack[self.sp - 2] -= self.stack[self.sp - 1];
                self.sp -= 1;
            }
            InstType::Mul => {
                if self.sp < 2 {
                    return Err(Error::StackUnderflow);
                }
                self.stack[self.sp - 2] *= self.stack[self.sp - 1];
                self.sp -= 1;
            }
            InstType::Div => {
                if self.sp < 2 {
                    return Err(Error::StackUnderflow);
                }
                else if self.stack[self.sp - 2] == 0 || self.stack[self.sp - 1] == 0 {
                    return Err(Error::DivByZero);
                }

                self.stack[self.sp - 2] /= self.stack[self.sp - 1];
                self.sp -= 1;
            }
            InstType::Jmp => {
                if inst.operand as usize > self.program.len() || inst.operand < 0 {
                    return Err(Error::IllegalJmp);
                }
                
                self.ip = inst.operand as usize;
            }
            InstType::Cmp => {
                if self.sp < 2 {
                    return Err(Error::StackUnderflow);
                }
                
                let result = if self.stack[self.sp - 2] == self.stack[self.sp - 1] { 1 } else { 0 };

                if self.sp < STACK_CAP {
                    self.stack[self.sp] = result;
                }
                else {
                    return Err(Error::StackOverflow);
                }
            }
            InstType::Store => {
                if inst.operand as usize >= MEMORY_CAP || inst.operand < 0 {
                    return Err(Error::SegmentationFault);
                }
                if self.sp < 1 {
                    return Err(Error::StackUnderflow);
                }

                self.sp -= 1;
                self.memory[inst.operand as usize] = self.stack[self.sp];
            }
            InstType::Load => {
                if inst.operand as usize >= MEMORY_CAP || inst.operand < 0 {
                    return Err(Error::SegmentationFault);
                }
                if self.sp < 1 {
                    return Err(Error::StackUnderflow);
                }
                
                self.stack[self.sp] = self.memory[inst.operand as usize];
                self.sp += 1;
            }
            InstType::Halt => {
                self.halt = true;
            }
        }

        Ok(())
    }

    fn dump(&self) {
        println!("Stack:");

        if self.sp < 1 {
            println!("  [empty]");
        }
        else {
            for i in 0..self.sp {
                println!("  {} - {}", self.stack[i], i);
            }
        }
    }
}

// TODO: Load Program from file (.cvm)

fn main() -> Result<(), Error> {

    let program = vec![
        Inst::new(InstType::Push, 69),
        Inst::new(InstType::Push, 2),
        Inst::new(InstType::Mul, 0),
        Inst::new(InstType::Push, 2),
        Inst::new(InstType::Div, 0),
        Inst::new(InstType::Pop, 0),
    ];

    let mut machine = Machine::new(program);
    machine.debug = true;
    let _ = machine.exec()?;

    Ok(())
}
