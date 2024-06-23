
#[derive(Debug)]
pub enum Error { 
    StackOverflow,
    StackUnderflow,
    SegmentationFault,
    OutOfMemory,
    IllegalInst,
    DivByZero,
    IllegalJmp,
}