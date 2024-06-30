# cvm46

64-bit Virtual Machine

## Instructions

| Opcode | Description             |
|--------|-------------------------|
| Pushi  | Push Integer on Stack   |
| Pushf  | Push Float on Stack     |
| Pushd  | Push Double on Stack    |
| Pushc  | Push Char on Stack      |
| Pushr  | Push Register on Stack  |
| Pop    | Pop value off Stack     |
| Popr   | Pop value off stack to register |
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
| Jeq    | Jump if true            |
| Jne    | Jump if false           |
| Halt   | Halt Execution          |
| Call   | Jumps to create new Stack Frame |
| Return | Jumps back to previous Stack Frame and ip |
| Exit   | Exit and Stop Execution |
| Cmp    | Compare Top of Stack    |
| Alloc  | Allocate Memory         |
| Free   | Free Memory             |
| Set    | Set Element             |
| Mov    | Mov to register         |
| Loadr  | Load Register from Heap          |
| Storer | Store Register in Heap         |
| Open   | Open File               |
| Close  | Close File              |
| Readf  | Read File               |
| Writef | Write File              |
| Read   | Read Stdin              |
| Write  | Write Stdout            |

**More coming**
