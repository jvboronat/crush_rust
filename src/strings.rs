// Primitive str = Inmutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run ()
{

    let hello = "Inmutable";    

    let mut hello1 = String::from("Hello World");

    // Get length
    println!("hello length {}", hello1.len());

    hello1.push(' '); // only for one caracter

    hello1.push_str("added string");

    println!("{} and {}", hello, hello1);
       
    // Capacity in bytes
    println!("Capacity: {}", hello1.capacity());

    // Check if empty
    println!("Is Empty: {}", hello1.is_empty());

    // Contains
    println!("Contains 'World' {}", hello1.contains("World"));

    // Replace
    println!("Replace {}", hello1.replace("World", "There"));

    // Loop through string by whitepace
    for word in hello1.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity

    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    


}