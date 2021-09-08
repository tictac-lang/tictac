# tictac
TicTac is a language written in Rust meant to be fast and simple. It has two modes, compiler and evaluator. The compiler one compiles to assembly that gets built by the assembler (GNU Assmebler) and the evalutor runs on runtime. For both of them, they go through Lexer and Generalize so output will be similar, if not the same.

### Contributers
[@actuallyexeon(main owner)](https://github.com/actuallyexeon)
[@datkat21(helped with vscode support)](https://github.com/datkat21)

### Syntax
Function Definition: 
```rust
(function_name) -> {function_arguments}|{
    // code
}
```
(eg.):
```rust
(main) -> {}:int|{
    =msg <- "Hello, World";
    (eprint) -> {=msg};
}
```
Function running: ``(function_name) -> {function arguments}`` (eg.): ``(eprint) -> {"Hello"}``