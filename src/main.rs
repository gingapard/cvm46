type Word = i64;
const STACK_CAP: usize = 1024;

enum Error {
    StackOverflow,
    StackUnderflow,
    SegmentationFault,
    IllegalInst,
}

enum InstType {
    Push,
    Plus,
    Sub,
    Mul,
    Div,
}

struct Inst {
    inst_type: InstType,
    operand: Word
}

impl Inst {
    fn new(inst_type: InstType, operand: Word) -> Self {
        Inst { inst_type, operand }
    }
}

struct Machine {
    stack: [Word; STACK_CAP],
    stack_size: usize,
}

impl Machine {
    fn new() -> Self {
        Machine { stack: [0; STACK_CAP], stack_size: 0 }
    }

    fn exec(&mut self, inst: &Inst) -> Result<(), Error> {
        
        match inst.inst_type {
            InstType::Push => {
                self.stack[self.stack_size] = inst.operand;
                self.stack_size += 1;
            },
            InstType::Plus => {
            },
            InstType::Sub => {
            },
            InstType::Mul => {
            },
            InstType::Div => {
            }
            _ => return Err(Error::IllegalInst)
        }

        Ok(())
    }

    fn dump(&self) {
        println!("Stack:");
        for i in 0..self.stack_size {
            println!("  {}", self.stack[i]);
        }
    }
}

fn main() {
    let mut machine = Machine::new();
    let inst = Inst::new(InstType::Push, 69);
    let _ = machine.exec(&inst);
    let _ = machine.dump();
}
