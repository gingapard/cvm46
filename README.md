# cvm46

64-bit Virtual Machine

## Instructions

| Opcode | Description             |
|--------|-------------------------|
| Pushi  | Push Integer on Stack   |
| Pushf  | Push Float on Stack     |
| Pushd  | Push Double on Stack    |
| Pop    | Pop value off Stack     |
| Dup    | Duplicate top of Stack  |
| Plus   | Plus top of Stack       |
| Sub    | Sub top of Stack        |
| Mul    | Mul top of Stack        |
| Div    | Div top of Stack        | 
| And    | Bitwise And             |
| Or     | Bitwise Or              |
| Xor    | Bitwise Xor             |
| Not    | Bitwise Not             |
| Jmp    | Change Inst Pointer     |
| Jeq    | Jumps if equals true    |
| Jne    | Jumps if equals false   |
| Cmp    | Compare value           |
| Store  | Store on Heap           | 
| Load   | Load from Heap           |
| Halt   | Halt Execution           |
| Call   | Jumps to create new Stack Frame |
| Return | Jumps back to previous Stack Frame and ip |
| Open   | Open File               |
| Close  | Close File              |
| Readf  | Read File               |
| Writef | Write File              |
| Read   | Read Stdin              |
| Write  | Write Stdout            |

**More coming**
