# Osta Lang Compiler - Rust Implementation
This is a Rust implementation of the Osta Lang Compiler (costa).
## What is Osta Lang?
Osta is a modern view of the well known C language.
## Roadmap
- [x] Lexer: File -> Token Stream
- [ ] Parser: Token Stream -> AST
- [ ] SSA: AST -> SSA
- [ ] Optimizer: SSA -> SSA
- [ ] IL: SSA -> IL
- [ ] Translator: IL -> IL
- [ ] Backend: IL -> Machine Code
## TODO:
- [ ] Expressions
  - [ ] FIXES: ([src](osta-parser/src/rules/expr.rs))
  - [ ] Assignment
- [x] If statements
- [ ] Loops
  - [x] Do-While
  - [ ] 🪄 While `if (condition) do { ... } while (condition)`
  - [ ] 🪄 For `{ init; while (condition) { ...; update } }`
- [ ] Statements
  - [ ] Variable declaration
  - [ ] Expression statement (`expr;`)
    - [x] Function call
  - [ ] Return
## Quirks
- Expressions:
    - `if <expr:bool> <expr:T> [else <expr:T>]` is an `<expr:T>`
    - `do <expr:T> while <expr:bool>` is an `<expr:T>`.
      **Note:** Returning a value stops the loop.
    - `while <expr:bool> <expr:T>` is an `<expr:T>`.
      **Note:** Returning a value stops the loop.
    - `{ <expr:T> }` is an `<expr:T>`.
    - `for <stmt> <expr:bool> <stmt> <expr:T>` is an `<expr:T>`.
      **Note:** Returning a value stops the loop.
    - `<ident> = <expr:T>` is an `<expr:T>`.
- Statements:
  - `<expr:*>;` is a `<stmt>`.
  - `<type:T> <ident> = <expr:T>;` is a `<stmt>`.
- Types:
  - `<ident> [<generic>]` is a `<type>`.
  - `<type> *` is a `<type>`.
  - `(<type>)` is equivalent to `<type>`. `(<type>,)` is a tuple.
- Annotations:
  - Annotations are instructions to the metacompilation pipeline.
  - `@<ident> [(<token stream>)]` is an `<annotation>`.
## Special Annotations - WIP
- `@cotype(T)` - Multi-instance types (`T*`, `T[]`, `&T[]`, etc.) of generic types (`S<A<T>, B<T>>`) can be used as follows:
  ```osta
  @cotype(T) Map<A<T>, B<T>> map = new Map<A<T>, B<T>>();
  
  A<u8> a1 = new A<u8>();
  B<u8> b1 = new B<u8>();
  A<u16> a2 = new A<u16>();
  B<u16> b2 = new B<u16>();
  
  // Map<A<?>, B<?>>.put<T>(A<T>, B<T>)
  
  map.put(a1, b1);
  map.put(a2, b2);
  
  // Map<A<?>, B<?>>.get<T>(A<T>) -> B<T>
  
  B<u8> b1_ = map.get(a1);
  B<u16> b2_ = map.get(a2);
  ```