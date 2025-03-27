# Bifrost

Bifrost is an evolution to [thorlang](https://github.com/Adotweb/thorlang), an interpreted language written in rust in the scope of a Matura paper.
Bifrost aims to improve upon thorlang by using concepts and adding static types and a typechecker as well as (hopefully) a compiler backend.

## Roadmap
- [x] Lexer
- [ ] Parser
- [ ] Final Syntax Design
- [ ] Interpreter
- [ ] Typechecker
- [ ] Compiler
- [ ] Quality of Life Improvements



## Syntax
The Idea for Bifrost is to make programming intuitive and easier to pickup as well as to provide a good language for intermediate programmers that try to get into compiled and static languages.

The syntax tries to optimize for ease of use: 

### Variables
To declare variables we use the `let` or `const` keywords:
```thorlang 

// let is reassignable
let a = 5;
a = 6;

//const isn't
const b = 6;

// both can be type annotated: 
let c : string = "hello";
const d : number = 0;
```


