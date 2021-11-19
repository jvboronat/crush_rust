// Variables hold primitive data or references to data
// Variables are inmutable by default
// Rust is block-scoped language (about variables)

pub fn run(){
    let name = "Brad";
    let mut age = 37; // This variable is MUTABLE now

    println!("My name is {} and I am {}", name, age);
    age = 38;

    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID:i32 = 001;
    println!("ID: {}", ID); // When we define a constant we must specify the type


    // Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);

    println!("{} is {}",my_name, my_age);

}
