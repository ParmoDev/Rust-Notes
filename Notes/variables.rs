// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "John Doe"; // Normal variable (immutable)
    let mut age = 30; // Mutable variable

    println!("{} is cool! He is {} years old.", name, age);

    age = 31; // Re-assign a mutable variable

    println!("{} is cool! He is {} years old.", name, age);

    // Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let ( my_name, my_age ) = ("John Doe", 30);
    println!("Hello my name is {} and I am {} years old", my_name, my_age);
}