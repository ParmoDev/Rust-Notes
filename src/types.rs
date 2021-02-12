// Datatypes
/*
Primitive Types:
Integers: u8, i8, u16, u32, i32, u64, i64, u128, i128, (Number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language which means that it must know the types of all variables at compile time
// However, the compiler can usually infer what we want to use based on the value and how we use it.

pub fn run() {
    // Default is 'i32'
    let x = 1;

    // Default is 'f64'
    let y = 2.5;

    // Add explicit type
    let z: i64 = 9223372036854775807;

    // Find max size of a type
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 9;

    // Character. Notice single quotes
    let a1: char = 'a';
    let face = '\u{1F600}'; // Characters are unicode meaning you can put emojis!!!!

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}