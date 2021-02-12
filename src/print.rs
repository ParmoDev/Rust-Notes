// Creating a public (pub) function (fn) with the name 'run'
pub fn run() {
    println!("Hello, World!"); // Printing a string to the console.

    // Formatting
    println!("Cool Number: {}", 123); // Printing an integer to the console.
    println!("A bunch of numbers: {}, {}, {}", 14, 16, 945); // Printing multiple integers to the console.
    println!("{} is from {}", "John Doe", "Canada"); // Works the same way with strings/variables!

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "John Doe", "Canada" , "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name="John Doe", activity="video games");

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "Hello, World!")); // Also known as a 'tuple'

    // Basic math
    println!("10 + 10 = {}", 10 + 10)

}