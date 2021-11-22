// Array - Rezisable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5000000]; // By default are inmutable

    // Re-assign value
    numbers[2] = 20;

    println!("Original {:?}", numbers);        

    // Add on to vector
    numbers.push(5);

    println!("Added 5, result : {:?}", numbers);         

    numbers.push(6);

    println!("Added 6, result : {:?}", numbers);             

    numbers.pop();

    println!("Popped 6, result : {:?}", numbers);             

    // Single value
    println!("Single value {}", numbers[0]);

    println!("");

    // Array are stack allocated

    println!("Vector occupies {} bytes",mem::size_of_val(&numbers));    

    // Get Slice

    let slice: &[i32] = &numbers[0..2];

    println!("Slice {:?}", slice);

    //Loop
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //Loop & Mutate
    for x in numbers.iter_mut() {
        *x *= 2;

        
    }   
    
    println!("Number Vec: {:?}", numbers);
    

}