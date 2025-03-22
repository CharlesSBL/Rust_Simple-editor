// Make a simple terminal editor
// 0.0.1 Make it read the file
// 0.0.2 Make it edit the file
// 0.0.3 Make it save the file
// 0.0.4 Make it create the file

use std::fs::File;
use std::io::{self, Read};

fn main() {
    if let Err(e) = read_file() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

// to read file we need to

fn read_file() -> io::Result<()> {
    let path: String = String::from("file.txt");

    let mut file: File = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return Err(e);
        }
    };

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(_) => println!("File content loaded: {} bytes", content.len()),
        Err(e) => {
            println!("Error reading file: {}", e);
            return Err(e);
        }
    }

    println!(
        "\n┌───────────────────────────────┐\n│       File Content            │\n├───────────────────────────────┤\n\n{}\n\n└───────────────────────────────┘\n",
        content
    );

    Ok(())
}
