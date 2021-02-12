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

    // Get length
    println!("Length: {}", hello.len());

    println!("{}", hello2);
}