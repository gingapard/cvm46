
#[derive(Debug)]
pub enum Error { 
    StackOverflow,
    StackUnderflow,
    SegmentationFault,
    IllegalInst,
    DivByZero,
    IllegalJmp,
}
