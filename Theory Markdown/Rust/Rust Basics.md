Rust basics
===========

Dependencies
------------
`cargo new project-name`
Cargo.toml:

```
rand = "0.8.5"
```

running;
`cargo build`
will download and then compile the “rand” plugin with version 0.8.5 and place this in Cargo.lock 
  

running;
`cargo update`
will update “rand”, but ignore any version starting from v0.9.0
Updating crates.io index
Updating rand v0.8.5 -> v0.8.6
  

To make sure it installs the newest version, like v0.9.0, we have to change that in Cargo.toml:

```
rand = "0.9,0"
```
  

### Prelude

The Rust standard library is imported into the scope of every program by default. This contains a set of items which we call the *prelude*

### String type

```
let mut guess = String::new();  
```

*“String”* is a String type provided by the standard library that is a growable, UTF-8 encoded bit of text.
  
### ::

*“::”* is a piece of Rust syntax that indicates a function associated to the Type at its left-hand side.
  

### importing libraries and modules 


  
```
use std::io;
```

imports the *io* library (allows user input / output) from the standard library (std, which does not have to be manually installed).
  
```
use rand::Rng;
```

imports the `Rng` trait from the `rand` crate. 
  

References
----------


  
```
let mut guess = String::new();  


 io::stdin()
 .read_line(&mut guess)
  
```

The *&* symbol signifies a reference, in this case referencing to the declared mutable variable “guess”. We use this to access this variable without having to copy its data into memory another time. We do, however, need to specify that its value is mutable again by passing the *mut* keyword to it.
  

Enumerations
------------


  

An enumeration – *enum* – is a type that can be in one of multiple states. Each possible state is called a variant.
  

### .expect()
  
```
io::stdin()
 .read_line(&mut guess)
 .expect("Failed to read line");
```
  
`.readline()` returns a `Result` enumeration, which has the variants `Ok` and `Err`.
If, in this case, `.readline()` executes successfully, it returns an `Ok` variant of the `Result` enum, containing the value of this method.

However, if `.readline()` returns an `Err` variant of the `Result` enum, the program won’t compile as this error has to be handles.  

`.expect()` is a method that returns the contents of the `Result` enum. In the case of `Ok`, it returns the read value, in the case of `Err`, it will cause the program to crash and return the passed the string specified in its brackets. 

Documentation
-------------

`cargo doc --open`
is a useful command that builds and opens a documentation file for all used dependencies in your project.
  
Match
-------

A match expression is made up of **arms** that contain a pattern to match against and a piece of code to execute if the passed value matches the pattern.

```
use std::io;
use std::cmp::Ordering

fun main () {

	let secret_number = rand::thread_rng().gen_range(1..=100);

	let mut guess = String::new();  

	io::stdin()
	 .read_line(&mut guess)
	 .expect("Failed to read line");

	match guess.cmp(&secret_number) {
		Ordering::Less => println!("Too small!") // arm 1
		Ordering::Greater => println!("Too big!") // arm 2
		Ordering::Equal => println!("Equal!") // arm3
	}
}
```

This would return an error as the values *guess* and *secret_number* have mismatched types. Fix this by adding the following line above `match`:

```
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

`trim` eliminates whitespace, `parse` compares the given value to another type and and converts it if successful – in this case it compares a `String` type to a `u32` type, as annotated. The type of *secret_number* inside the match expression is inferred by comparing it to guess, which is a `u32` type.

Loop
------

We can modify the code as follows to loop the program until a correct guess is made.
`loop {}`  loops the enclosed logic, `continue` makes it skip to the next iteration of the loop.

```
loop {
	// code
	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	let guess: u32 = match guess.trim().parse() {
		Ok(num) => num,
		Err(_) => continue,
  };
	
	match guess.cmp(&secret_number) {
		Ordering::Less => println!("Too small!")
		Ordering::Greater => println!("Too big!")
		Ordering::Equal => {
			println!("You win!")
			break;
		}
	}
}
```
### Loop labels
In certain cases it can be useful to name loops so they can be referenced from inside. Such as the following.
```
fn main () {
	let mut counter = 0;

	'a: loop {
		'b: loop {
			if counter == 2 {
				println!("counter == 2 and won't be incremented");
				continue 'a;
			}
			counter += 1;
			println!("loop 'b is run with counter == {}", counter);
		}
		println!("loop 'a was run, counter == {}", counter);
		if counter == 2 {
			break;
		}
	}
}
```