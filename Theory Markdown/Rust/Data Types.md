Data Types
==========
Rust must know the type of every variable at compile time. In the case of ambiguous declarations like such;
```
let guess = “43”.parse().expect(“Not a number!”)
```
Rust will give an error as there are many types possible
```
let guess: u32 = “43”.parse().expect(“Not a number!”)
```  

Subsets: Scalar
---------------
Represents a single value.
* integers
* floating-point numbers
* booleans
* characters

### Integers
Integers can be unsigned or signed and are specified with a bit size:
|  |  |  |
| --- | --- | --- |
| **Length** | **Signed** | **Unsigned** |
| 8-bit | i8 | [u8](https://doc.rust-lang.org/std/primitive.u8.html#) |
| 16-bit | i16 | [u16](https://doc.rust-lang.org/std/primitive.u16.html) |
| 32-bit | i32 | [u32](https://doc.rust-lang.org/std/primitive.u32.html) |
| 64-bit | i64 | [u64](https://doc.rust-lang.org/std/primitive.u64.html) |
| 128-bit | i128 | [u128](https://doc.rust-lang.org/std/primitive.u128.html) |
| arch | isize | [usize](https://doc.rust-lang.org/std/primitive.usize.html) |

Signed integers are ‘signed’; they can hold negative values if signed with a minus-sign.

**Signed storage size:**
$-(2^{n-1})$ to $2^{n-1} - 1$ 
for **i8**:
$-(2^7)$ to $2^7-1$
$-128$ to $127$

**Unsigned storage size:**
$2^n - 1$
for **i8**:
$2^8-1$
$0$ to $255$

### Integer Overflow
Integer overflow occurs when a variable of an integer gets a value assigned that doesn't fit in the range of its type.

**Debug mode**
In debug mode, the program includes type checking and "panics" when an integer overflows; the program exits with an error.

**Release mode**
When compiling in release mode with the `--release` flag, Rust doesn't include checks for integer overflow but instead performs **two's complement wrapping**; values greater than the possible range wrap around to the minimum values of their range and count from there.
i.e. for **u8** $257$ becomes $1$

**Handling integer overflow**
`wrapping_...` returns a wrapped value
`overflowing_...` returns a tuple containing a wrapped value and a boolean indicating whether the value was wrapped or not
`saturating_...` returns the maximum value of the range's type
`checked_...` returns 'Some(result)' if no overflow occured and 'None' if overflow did occur
`std::num::Saturating` 	

<details>
	<summary><code>assert_eq!(255u8.wrapping_add(1), u8::MIN)</code></summary>

This is using an assertion to check whether the result of `255u8.wrapping_add(1)` is equal to` u8::MIN`. If the assertion fails, it will panic and indicate that the values are not equal. This approach is commonly used for testing and ensuring specific behaviors in your code.
</details>

<details>
	<summary><code>u8::MAX.wrapping_add(1) == 0u8</code></summary>
This is a direct comparison that checks whether the result of `u8::MAX.wrapping_add(1)` is equal to `0u8`. It's not using an assertion, so it won't panic if the condition is false. Instead, it will return a boolean result (`true` if equal, `false` if not).
</details>

<details>
	<summary><code>let a = Overflowing(u8::MAX); (a + 1).0 == u8::MIN</code></summary>
This code snippet appears to be using a custom type called `Overflowing` to represent integer overflow behavior. The `Overflowing` type seems to return a tuple where the first element is the wrapped value. This code creates an instance of `Overflowing` with `u8::MAX`, adds 1 to it, and checks if the first element of the resulting tuple is equal to `u8::MIN`. This approach provides more control and encapsulation for handling overflow but requires defining the `Overflowing` type.
</details>


All three options are valid and check whether adding 1 to the maximum value of u8 wraps around to the minimum value. The first two are simpler and more common, while the third one involves creating a custom type, which might be suitable for specific use cases where custom overflow handling is required.

### Floating-points
`f32`: single-precision float
`f64`: double-precision float
The default floating-point type in Rust is `f64` as the speed to calculate it on modern CPUs is roughly the same as that of `f32` whilst being more accurate.

### Numeric Operators
Sum `+`, subtraction `-`, division `/`, multiplication `*` and remainder `%`.
Division truncates downwards.
[List of all operators in rust](https://rust-book.cs.brown.edu/appendix-02-operators.html)

### Booleans
Booleans are one byte in size and are annotated using the `bool` keyword.
Values are either `true` or `false`.

### Characters
Characters are the most primitive alphabetic type and represent a single Unicode Scalar Value (ranges from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF`).
Characters are specified using **single quotes** as opposed to double quotes in String literals and are specified using `char`.

Subsets: Compound
--
Compound types group multiple values into one type type.

### Tuples
Tuples can group together multiple values with a variety of different types.
```
let tup: (i32, u8, char) = (650, 2, 'O')

let (x, y, z) = tup; // destructuring

let x = tup.0; // shadow variable by accessing tuple element on index 0
```

**Unit value**
A tuple without a value `()` is called a unit and represents an empty value / return type. Any expression that doesn't explicitly return anything returns a unit value.

### Arrays
Arrays group together multiple values of the same type. Arrays have a **fixed length**.
Arrays allocate data in the [stack](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html?highlight=stack#variables-live-in-the-stack) rather than the [heap](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html?highlight=heap#boxes-live-in-the-heap) and are useful when having to ensure a consistent fixed number of elements.
When in need of a more flexible list (arrays like those in JavaScript), use the [vector](https://rust-book.cs.brown.edu/ch08-01-vectors.html) collection type.

An array is declared and initialized as follows:
```
let arr: [i32; 5] = [400, 4, 0, 37, 33]
```
And can also be initialized as such, wherein its contained type is inferred upon initialization:
```
let arr = [3; 5]; // arr's type is now [i32; 5] and holds value 3 on each index
```

On invalid element array access, Rust will "panic" and exit the program with an error.
**code**:
```
let arr = [3; 5];

let x = arr[5];
```
**error**:
```
Compiling playground v0.0.1 (/playground)
error: this operation will panic at runtime
 --> src/main.rs:5:9
  |
5 | let x = arr[5];
  |         ^^^^^^ index out of bounds: the length is 5 but the index is 5
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: could not compile `playground` (bin "playground") due to previous error

```

If Invalid array element access occurs at runtime, the program exits too:

**code**:
```
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```
**error**:
```
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
