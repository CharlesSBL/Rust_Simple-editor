```rust
// Rust doesn't have classes and interfaces in the traditional OOP sense,
// but we can use traits and structs to achieve similar patterns

// Define a trait (similar to an interface)
trait Animal {
    fn make_sound(&self) -> String;
    fn get_name(&self) -> &str;
}

// Define structs (similar to classes)
struct Dog {
    name: String,
    breed: String,
}

struct Cat {
    name: String,
    color: String,
}

// Implement the Animal trait for Dog
impl Animal for Dog {
    fn make_sound(&self) -> String {
        format!("{} the {} dog says: Woof!", self.name, self.breed)
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

// Implement other methods for Dog
impl Dog {
    // Constructor (similar to a class constructor)
    fn new(name: String, breed: String) -> Self {
        Dog { name, breed }
    }

    fn get_breed(&self) -> &str {
        &self.breed
    }
}

// Implement the Animal trait for Cat
impl Animal for Cat {
    fn make_sound(&self) -> String {
        format!("{} the {} cat says: Meow!", self.name, self.color)
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

// Implement other methods for Cat
impl Cat {
    // Constructor
    fn new(name: String, color: String) -> Self {
        Cat { name, color }
    }

    fn get_color(&self) -> &str {
        &self.color
    }
}

// Function that works with any type that implements Animal trait (polymorphism)
fn animal_sounds(animals: Vec<&dyn Animal>) {
    for animal in animals {
        println!("{}: {}", animal.get_name(), animal.make_sound());
    }
}

fn main() {
    // Create instances
    let dog: Dog = Dog::new(String::from("Buddy"), String::from("Golden Retriever"));
    let cat: Cat = Cat::new(String::from("Whiskers"), String::from("Orange"));

    // Use polymorphism with a vector of trait objects
    let animals: Vec<&dyn Animal> = vec![&dog, &cat];
    animal_sounds(animals);

    // Access specific methods
    println!("Dog breed: {}", dog.get_breed());
    println!("Cat color: {}", cat.get_color());
}

```
