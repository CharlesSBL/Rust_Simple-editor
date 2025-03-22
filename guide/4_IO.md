# Rust Input/Output Workflow Guide

Rust provides several ways to handle input and output operations through its standard library. This guide will walk you through common I/O patterns with examples and explanations.

## Basic Console Input/Output

```rust
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Print to console without a newline
    // flush() ensures the text is immediately displayed
    print!("Enter your name: ");
    io::stdout().flush()?;

    // Create a new empty String to store input
    let mut input = String::new();

    // Read a line from standard input (keyboard)
    // The input is stored in the input variable
    io::stdin().read_line(&mut input)?;

    // Trim whitespace and newlines from the input
    let input = input.trim();

    // Print the input with a newline
    println!("Hello, {}!", input);

    Ok(())
}
```

## Reading from Files

```rust
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Open a file for reading
    // This returns a Result type that we handle with ?
    let mut file = File::open("example.txt")?;

    // Create a String to store the file contents
    let mut contents = String::new();

    // Read the entire file into the string
    file.read_to_string(&mut contents)?;

    println!("File contents:\n{}", contents);

    // Shorter way to read a file to string
    let contents = std::fs::read_to_string("example.txt")?;

    println!("File contents (method 2):\n{}", contents);

    Ok(())
}
```

## Writing to Files

```rust
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Create a new file (or truncate an existing one)
    let mut file = File::create("output.txt")?;

    // Write a string to the file
    write!(file, "Hello, world!\n")?;

    // Write multiple lines
    writeln!(file, "This is line 1")?;
    writeln!(file, "This is line 2")?;

    // Shorter way to write a string to a file
    std::fs::write("output2.txt", "Hello, Rust I/O!")?;

    println!("Files have been written successfully!");

    Ok(())
}
```

## Handling Command Line Arguments

```rust
use std::env;

fn main() {
    // Collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // The first argument (args[0]) is the program name
    println!("Program name: {}", args[0]);

    // Check if we received any additional arguments
    if args.len() > 1 {
        println!("Arguments received:");

        // Iterate through all arguments except program name
        for (i, arg) in args.iter().enumerate().skip(1) {
            println!("  Argument {}: {}", i, arg);
        }
    } else {
        println!("No arguments provided");
    }
}
```

## Reading User Input with Error Handling

```rust
use std::io::{self, Write};

fn main() {
    // Loop until we get valid input
    let number = loop {
        print!("Please enter a number: ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Create a new String for this attempt
        let mut input = String::new();

        // Read input
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input, please try again");
            continue;
        }

        // Parse the input as a number
        // If parse succeeds, break from loop with the value
        // If parse fails, show error and continue loop
        match input.trim().parse::<i32>() {
            Ok(num) => break num,
            Err(_) => println!("That's not a valid number, please try again"),
        }
    };

    println!("You entered: {}", number);
    println!("Twice that is: {}", number * 2);
}
```

## Working with JSON

```rust
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

// Define a struct that can be serialized/deserialized
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
    emails: Vec<String>,
}

fn main() -> io::Result<()> {
    // Create a Person instance
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        emails: vec!["alice@example.com".to_string(), "alice.work@example.com".to_string()],
    };

    // Serialize to JSON string
    let json = serde_json::to_string_pretty(&person)
        .expect("Failed to serialize to JSON");

    // Print the JSON
    println!("JSON representation:\n{}", json);

    // Write the JSON to a file
    let mut file = File::create("person.json")?;
    file.write_all(json.as_bytes())?;

    // Read the JSON from the file
    let mut file = File::open("person.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize from JSON string
    let loaded_person: Person = serde_json::from_str(&contents)
        .expect("Failed to deserialize JSON");

    println!("Deserialized person: {:?}", loaded_person);

    Ok(())
}
```

## Conclusion

This guide covered the basic input/output workflows in Rust, including:
- Console I/O
- File operations
- Command-line arguments
- User input with error handling
- JSON serialization/deserialization

Remember that most I/O operations in Rust return a `Result` type, allowing you to handle errors gracefully using `?` or pattern matching.
