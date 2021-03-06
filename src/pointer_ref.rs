// Reference Pointers - Point to a resource in memory

pub fn run () {

    // Primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // With non-primitives, if you assign another variable to a piece of data, the first
    // variable will no longer hold that value. You'll need to use reference (&) to point to the 
    // resource

    // Vector array
    let vect1 = vec![1,2,3];
    let vect2 = &vect1;    

    println!("Values: {:?}", (&vect1, vect2));

    

}