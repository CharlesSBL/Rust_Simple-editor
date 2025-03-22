## rust
## 3. Control Flow

### if/else expressions
```rust
// Basic if/else expression
let number = 7;
if number < 5 {
    println!("The number is less than 5");
} else if number == 5 {
    println!("The number is equal to 5");
} else {
    println!("The number is greater than 5");
}

// if/else expressions return values
let condition = true;
let result = if condition {
    "condition was true"  // No semicolon here as this is the return value
} else {
    "condition was false" // Both branches must return same type
};
println!("The result is: {}", result);
```

### Loops (loop, while, for)
```rust
// 1. Infinite loop with 'loop'
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        // Break can return a value from a loop
        break counter * 2;
    }
};
println!("The result is: {}", result); // Prints 20

// 2. While loop
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}
println!("LIFTOFF!!!");

// 3. For loop over a range
for num in 1..4 {  // Range is exclusive of upper bound
    println!("{}!", num);
}
println!("LIFTOFF!!!");

// 4. For loop over a collection
let animals = vec!["rabbit", "dog", "cat"];
for animal in animals.iter() {
    println!("The animal is: {}", animal);
}
```

### Match expressions
```rust
// Basic match expression
let number = 3;
match number {
    1 => println!("One!"),
    2 => println!("Two!"),
    3 => println!("Three!"),
    _ => println!("Something else!"), // _ is a catch-all pattern
}

// Match with complex expressions
let result = match number {
    1 => "one",
    2 => "two",
    3 => "three",
    _ => "something else",
};
println!("Number is {}", result);

// Match with multiple patterns
let x = 1;
match x {
    1 | 2 => println!("One or two"),
    3..=5 => println!("Three to five"),
    _ => println!("Something else"),
}
```

### Pattern matching
```rust
// Destructuring structs
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };
let Point { x, y } = p;  // Destructuring
println!("x: {}, y: {}", x, y);

// Pattern matching with if let
let some_value = Some(3);
if let Some(3) = some_value {
    println!("Three!");
}

// Pattern matching with while let
let mut stack = Vec::new();
stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("Popped: {}", top);
}

// Destructuring tuples
let tuple = (1, "hello", 3.14);
let (a, b, c) = tuple;
println!("a: {}, b: {}, c: {}", a, b, c);
```
