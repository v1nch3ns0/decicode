# decicode
my stack-based vm esolang written in rust in under 100 lines of code made to be very compact and consistent in the code and it is turing-complete.
## opcodes

```
>    PUSH: Push a decimal literal immediately following '>' onto the stack. Example: >72 pushes 72.
!    POP: Pop the top value from the stack and discard it.
+    ADD: Pop the top value into a, pop the next value into b, push (b + a) back onto the stack.
*    MUL: Pop the top value into a, pop the next value into b, push (b * a) back onto the stack.
-    SUB: Pop the top value into a, pop the next value into b, push (b - a) back onto the stack.
/    DIV: Pop the top value into a, pop the next value into b, push (b / a) back onto the stack.
&    DUP: Duplicate the top value of the stack.
^    OUT: Peek the top value and print it as an ASCII character.
#    NOP: No operation.
?    CJMP: Conditional jump. If top of stack == 0, jump to the slot index given by the following literal.
:    JMP: Absolute jump to the slot index given by the following literal.
~    SWAP: Swap the two top values on the stack.
```

### slots

decicode treats **every single character in the program as a “slot”**. this includes:

- opcodes (e.g., `>`, `!`, `+`)  
- digits in literals (e.g., `7`, `2` in `>72`)  

each slot is **one unit of execution**. when you push a literal, each digit counts as a slot, but the VM reads all digits following `>` until it encounters a non-digit.  

this system keeps the language visually compact, consistent and allows you to jump to exact slot indexes removing the need for tokenizing and parsing instructions separately because every character is a step.
### examples
here is a program that loops printing 'H' 5 times in the main stack:

```>5&?16>72^!>1-:2#>10^```

now, this might look like gibberish until i explain how it works:

- `>5` pushes 5 to the stack so the stack should be: [ 5 ]
- `&` copies the top of the stack, which is 5 so the stack should be: [ 5, 5 ]
- `?16` condition jump, if the top of the stack is 0 jump to 16 (end of loop)
- `>72^!` might look weird but basically push 72 to the stack (H in ascii) and output it and pop the top of the stack so our stack still looks like: [ 5, 5 ]
- `>1-` push 1 to the stack and subtract the second most top of the stack by the top of the stack (in this case 5 - 1)
- `:2` simply jump back to the start of the loop
- `#>10^` the end of the loop is just the NOP and print a \n and exit

see now maybe its *kinda* easier?
### why you should use it and how to
it is **VERY** compact and simple while being turing complete at the same time which is very cool!!
to start using it just run `cargo build --release` and when you have created your program just run it with:
- `decicode yourfile`
