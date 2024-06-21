type Word = i64;
const STACK_CAP: usize = 1024;

#[derive(Debug)]
enum Error { StackOverflow,
    StackUnderflow,
    SegmentationFault,
    IllegalInst,
    DivByZero,
    IllegalJmp,
}

#[derive(Debug, Clone)]
enum InstType {
    Push,
    Pop,
    Plus,
    Sub,
    Mul,
    Div,
    Jmp,
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
    ip: usize,
    program: Vec<Inst>,

    // Makes it dump the stack on instruction execs
    debug: bool
}

impl Machine {
    fn new(program: Vec<Inst>) -> Self {
        Machine {
            stack: [0; STACK_CAP],
            sp: 0,
            ip: 0,
            program,

            debug: false
        }
    }

    /// Increments the ip and executes next instruction
    fn exec(&mut self) -> Result<(), Error> {
        while self.ip < self.program.len() { let inst = self.program[self.ip].clone();
            self.ip += 1;
            self.exec_inst(&inst)?;
            if self.debug { self.dump(); }
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
        Inst::new(InstType::Jmp, 0),
    ];

    let mut machine = Machine::new(program);
    machine.debug = true;
    let _ = machine.exec()?;

    Ok(())
}

