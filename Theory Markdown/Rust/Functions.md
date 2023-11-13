Functions
=
Rust is a functional programming language.
Functions are declared using the `fn` keyword and use *snake case* by convention.
`fn main()` is the entry point of a program.

Parameters
--
Functions can have parameters which are declared in their constructor and need a specified type.

### Note
**Parameters** are the declared variables with specified types in the constructor of the declared function.
**Arguments** are the actuall passed values upon function call.
```
fn main() {
	new_function(55) // passed argument
}
fn new_function(x: i64){ // declared parameter
	println!("X is a 64-bit signed integer with value {x}")
}
```

Expressions & Statements
--
Statements and expressions are two seperate concepts, as opposed to many other programming languages.

### Expressions
Expressions are assignments that have a return value.
Expressions are written **without** semicolon.

### Statements
Statements are code blocks that perform a set of actions and don't have a return value.
A statement is always followed by a **semicolon**.
Often, though, statements **contain** expressions, like so:
```
let y = 1;
let x = y + 6; // y + 6 is an expression that returns a value
```

Functions with return values
--
A function with a return type expects a return value that can either be returned by using an expression, or by using the `return` keyword.
```
fn new_function(x: i32) -> i32 {
	x + 3
}
```
This function contains an expression that returns the sum of the passed parameter and a value of 3.

Adding a semicolon to the expression would turn this line into a statement:
```
fn new_function(x: i32) -> i32 {
	x + 3;
}
```
However, the function still returns a value, in this case the **unit value** '()', which is of a wrong type!
```
Compiling functions v0.1.0 (file:///projects/functions)
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` due to previous error

```