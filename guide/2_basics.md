## Rust
## 2. Basic Syntax

### Variables and Mutability
```rust
// Immutable variable
let x = 5;
// x = 6; // Error: cannot assign twice to immutable variable

// Mutable variable
let mut y = 5;
y = 6; // OK

// Constants require type annotation
const MAX_POINTS: u32 = 100_000;

// Shadowing - reusing variable name
let z = 5;
let z = z + 1; // z is now 6
let z = "hello"; // z is now a string
```

### Data Types
```rust
// Integer types
let a: i8 = -128;    // 8-bit signed integer (-128 to 127)
let b: i16 = -32768; // 16-bit signed integer
let c: i32 = -42;    // 32-bit signed integer (default)
let d: i64 = -5000;  // 64-bit signed integer
let e: i128 = -12345678901234567890; // 128-bit signed integer
let f: isize = -100; // Pointer-sized signed integer (depends on system architecture)

let g: u8 = 255;     // 8-bit unsigned integer (0 to 255)
let h: u16 = 65535;  // 16-bit unsigned integer
let i: u32 = 42;     // 32-bit unsigned integer
let j: u64 = 5000;   // 64-bit unsigned integer
let k: u128 = 12345678901234567890; // 128-bit unsigned integer
let l: usize = 100;  // Pointer-sized unsigned integer

// Floating-point
let m: f32 = 3.14;   // 32-bit float (single precision)
let n: f64 = 2.71;   // 64-bit float (double precision, default)

// Boolean
let o: bool = true;  // Boolean (true/false)
let p = false;

// Character
let q: char = 'z';   // Single Unicode character (4 bytes)
let r = '😻';        // Unicode support

// String types
let s: &str = "hello"; // String slice (immutable, borrowed)
let t: String = String::from("world"); // Owned string (growable)

// Compound types
// Arrays - fixed size, same type
let u: [i32; 3] = [1, 2, 3];

// Tuples - fixed size, mixed types
let v: (i32, f64, char) = (42, 3.14, 'a');

// References
let w: &i32 = &c;    // Immutable reference
let x: &mut i32 = &mut c.clone(); // Mutable reference

// Other types
let y: () = ();      // Unit type (empty tuple)
let z: Option<i32> = Some(42); // Option enum
let aa: Result<i32, String> = Ok(42); // Result enum
```

### Compound Types
```rust
// Tuple - different types
let tup = (500, 6.4, 1);
let (x, y, z) = tup; // Destructuring
let first = tup.0;   // Access by index

// Arrays - same type, fixed length
let arr = [1, 2, 3, 4, 5];
let first_element = arr[0];

// Array with type and size
let arr: [i32; 5] = [1, 2, 3, 4, 5];

// Array with repeated values
let zeros = [0; 5]; // Creates [0, 0, 0, 0, 0]
```

### Functions and Statements
```rust
// Basic function
fn main() {
    println!("Hello, world!");
    let result = add(5, 6);
}

// Function with parameters and return value
fn add(x: i32, y: i32) -> i32 {
    x + y // No semicolon = return value
}

// Explicit return
fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

// Control flow
fn example() {
    // If expression
    let number = 6;
    if number % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }

    // Loop
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    // While loop
    while counter > 0 {
        counter -= 1;
    }

    // For loop
    for num in 1..5 {
        println!("{}", num); // Prints 1 to 4
    }

    // For loop with arrays
    let nums = [1, 2, 3];
    for n in nums.iter() {
        println!("{}", n);
    }
}
```

### Comments
```rust
// Single line comment

/* Block comment
   spanning multiple lines */

/// Documentation for the following function
fn documented_function() {
    // Function code here
}

/**
 * Block documentation
 * for the following item
 */
fn another_documented_function() {
    // Function code here
}
```
