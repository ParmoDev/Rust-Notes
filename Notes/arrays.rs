// Arrays - Fixed list where element are the same data types

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // Define type and len  Assign Values

    // Re-assign value - You cannot add onto arrays but you can re-assign values.
    numbers[1] = 20;

    println!("{:?}", numbers); // Prints out the entire array
    println!("Single Value: {}", numbers[1]); // Prints the 2nd element of the array

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes of memory", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}