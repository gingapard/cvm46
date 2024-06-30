# cvm46

64-bit Virtual Machine

## Instructions

| Opcode | Description             | Operands       |
|--------|-------------------------|----------------|
| Pushi  | Push Integer on Stack   | Immediate      |
| Pushf  | Push Float on Stack     | Immediate      |
| Pushd  | Push Double on Stack    | Immediate      |
| Pushc  | Push Char on Stack      | Immediate      |
| Pushr  | Push Register on Stack  | Register       |
| Pop    | Pop value off Stack     | None           |
| Popr   | Pop value off stack to register | Immediate |
| Dup    | Duplicate top of Stack  | Stack |
| Plus   | Plus top of Stack       | Stack |
| Sub    | Sub top of Stack        | Stack |
| Mul    | Mul top of Stack        | Stack |
| Div    | Div top of Stack        | Stack |
| And    | Bitwise And             | Stack |
| Or     | Bitwise Or              | Stack |
| Xor    | Bitwise Xor             | Stack |
| Not    | Bitwise Not             | Stack |
| Jmp    | Change Inst Pointer     | Immediate |
| Jeq    | Jump if true            | Stack & Immediate |
| Jne    | Jump if false           | Stack & Immediate |
| Halt   | Halt Execution          | Immediate |
| Call   | Jumps to create new Stack Frame | Immediate |
| Return | Jumps back to previous Stack Frame and ip | Immediate |
| Exit   | Exit and Stop Execution | Stack |
| Cmp    | Compare Top of Stack    | Stack |
| Alloc  | Allocate Memory         | Immediate |
| Free   | Free Memory             | Immediate |
| Set    | Set Element             | Stack & Immediate |
| Mov    | Mov to register         | Stack & Register  |
| Loadr  | Load Register from Heap | Immediate & Register |
| Storer | Store Register in Heap  | Register & Immediate |
| Open   | Open File               | Immediate |
| Close  | Close File              | Immediate |
| Readf  | Read File               | ... |
| Writef | Write File              | ... |
| Read   | Read Stdin              | None |
| Write  | Write Stdout            | None |

**More coming**
