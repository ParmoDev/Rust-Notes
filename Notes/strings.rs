// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when need to modify or own string data

pub fn run() {
    // By default is an str (primitive)
    let hello = "Hello";
    println!("{}", hello);

    // Growable string
    let mut hello2 = String::from("Hello!");
    hello2 = String::from("Hello");
    
    // Push a char to a string
    hello2.push(',');
    // Push a string to a string
    hello2.push_str(" World!");

    // Capacity in bytes
    println!("Capacity: {}", hello2.capacity());

    // Check if string is empty or not
    println!("Is Empty: {}", hello.is_empty());

    // Get length
    println!("Length: {}", hello.len());

    // Contains
    println!("Contains 'World' {}", hello2.contains("World"));

    // Replace
    println!("Replace: {}", hello2.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}, s capacity: {}", s, s.capacity());

    // Assertion testing
    assert_eq!(3, s.len()); // This will give an error, change it to 2 to remove error
    assert_eq!(11, s.capacity()); // Same deal change to 10 to remove error

    println!("{}", hello2);
}