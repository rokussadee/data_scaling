Enums and Pattern Matching
=

Using enums allows for grouping of different variants of a type by *enumerating* it. This is useful in cases where certain entities are associated to each other or part of the same abstraction whilst not necessarily sharing common characteristics. 

**Example**: 
Rectangles, circles and triangles are part of a set of shapes, but the properties that define them are not identical (width, height, radius and the necessary formulas used to calculate them).

### Advantages

Using `enums` allows for flexible usage of data types inside other functions. 
If we were to only make use of `structs` in the following function, we would have to implement an area calculation function for each struct.

```
struct Rectangle {
	w: f64,
	h: f64,
}

struct Triangle {
	b: f64,
	h: f64,
}

impl AreaCalculation for Rectangle {
	fn calculate_area(&self) -> f64 {
		self.w * self.h
	}
}

impl AreaCalculation for Triangle {
	fn calculate_area(&self) -> f64 {
		0.5 * self.b * self.h
	}
}
```

Implementing these shared functions for each type is much easier using enums.

Passing information
--

### Direct data attachment

```
enum Shape {
	Rectangle {w: f64, h: f64},
	Circle(f64), // tuple struct with single unnamed field implying a radius value
	Triangle {b: f64, h: f64},
	Trapezoid {w: f64, h: f64},
}
```

### Using structs

```
struct Rectangle {
	w: f64,
	h: f64,
}

// Idem for other shapes


enum Shape {
	Rectangle(Rectangle),
	Circle(Circle),
	Triangle(Triangle),
	Trapezoid(Trapezoid),
}

```

### Shared functions

```
struct Rectangle {
	w: f64,
	h: f64,
}

impl Shape {
	fn area(&self) {
		// method body
	}
}

let shape = Shape::Rectangle(Rectangle)
```

The Option enum
--

### Null values

Most programming languages have a **Null Value** type that indicates the absence of a value. Null values result in a lack of type safety for various reasons as listed hereunder.
<details>
	<summary>Null Pointer Exceptions</summary>
In languages that allow null values, attempting to dereference or use a null pointer can lead to runtime errors, such as null pointer exceptions. These errors can be challenging to debug and may result in program crashes.
</details>
<details>
	<summary>Unexpected Behavior</summary>
When null values are pervasive, it becomes easy to mistakenly assume that a variable always has a valid value. This assumption can lead to unexpected behavior if null values are not properly handled, potentially causing logical errors in the program.
</details>
<details>
	<summary>Code Complexity</summary>
Dealing with null values often requires additional code to check for null before performing operations. This can lead to increased code complexity, decreased readability, and a higher chance of introducing bugs.
</details>
<details>
	<summary>Maintenance Challenges</summary>
As the codebase grows, maintaining consistency in handling null values across different parts of the application can become challenging. Changes in one part of the code may inadvertently affect the behavior of other components.
</details>
<details>
	<summary>Lack of Type Safety</summary>
In languages where null is a valid value for any reference type, there is a lack of type safety. Developers may need to manually check and handle null values, increasing the risk of runtime errors.
</details>
<details>
	<summary>Optional Types</summary>
Some modern programming languages address these issues by introducing optional types or other constructs that explicitly handle the presence or absence of a value. This provides a safer and more explicit way to represent the potential absence of a value.
</details>

Rust deals with absent or invalid values by using the `Option` enumerator provided by its standard library.

### Rust's null values

It has two variants; a `None` variant that is used when a value is invalid or absent, and a`Some<T>` variant wherein `T` specifies the type of the value that should populate it in case an actual value is assigned.

```
enum Option<T> {
    None,
    Some(T),
}
```

Option 