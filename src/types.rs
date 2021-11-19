/* Primitive Types--
    Integers: u8, i8, ... u128, i128 (number of bits they take in memory)
            u: unsigned
            i: positives or negatives
    Floats: f32, f64
    Boolean (bool)
    Characters (char)
    Tuples
    Arrays

*/

// Rust is a statically type language, which means that it must know the type of all 
// variables at compiler time, however, the compiler can usually infer what type we 
// want to use based on the value and how we use it    (its not mandatory to set ype)


pub fn run ()
{
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    //Add explicit type
    let z: i64 = 12121212121;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active:bool = true;

    //Get a boolean form expresion
    let is_greater = 10 > 5;

    let a1 = 'a';

    let face = '\u{1F600}';

    println!("{:?}", (x,y,z, is_active, is_greater, a1, face));


}