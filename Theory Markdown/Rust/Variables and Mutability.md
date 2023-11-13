Variables and mutability
========================


  
```
let x = 5;
```

declares an immutable value x with value 5
  

changing this value like so:

```
x = 6;
```

will return an error:

error[E0384]: cannot assign twice to immutable variable `x`
  

to make ‘x’ mutable we use the ‘mut’ keyword:

```
let mut x = 5;
```
 
  

* data type does not have to be annotated
* can only be used in a function (not globally)


  

Constants
=========


  
```
const THREE\_HOURS\_IN\_SECONDS: u32 = 60 \* 60 \* 3;
```

declares a constant
  

* immutable by default
* declare with ‘const’
* data type must be annotated
* must be set on a constant expression (not depending on other variables)
* naming convention is IMMUTABLE\_CONSTANT
* constants CAN be globally scoped


  

Shadowing
=========


  

**Code**
  
```
let x = 5;

let x = x + 1;

{

 let x = x \* 2;

 println!(“{x}”)

}

println!(“{x}”)
```

**Output**
  

12

6
  

This is valid rust code.
Variable x gets redeclared by using ‘let’ again, essentially creating a new variable.
This is called shadowing
  

‘x’ gets shadowed once in the outer scope, and a second time in the inner scope which does not affect ‘x’ in the outer scope.
  

### “What is the difference with using a mutable variable?”


  

Reassigning a value to immutable variable ‘x’ will result in an error.
  

### “Why use this instead of the mutable keyword?"

Shadowing allows to change the type of the variable. It can also eliminate the need for new variable names;
  

let spaces = “ “;
let spaces = spaces.len()
  

First, ‘spaces’ is a string type, whereafter it gets shadowed and becomes a number type. This is useful in the case that the first value of ‘spaces’ wouldn’t be used in other parts of the code and only serves to create the actual spaces 
number.