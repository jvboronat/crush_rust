// Array - Fixed Where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5000000]; // By default are inmutable

    // Re-assign value
    numbers[2] = 20;

    // Single value
    println!("Single value {}", numbers[0]);

    println!("");

    // Array are stack allocated

    println!("Array occupies {} bytes",mem::size_of_val(&numbers));

    

    // Get Slice
    let slice: &[i32] = &numbers[0..2];

    println!("Slice {:?}", slice);

}