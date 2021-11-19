pub fn run() {
    //Print to conso

    //Formating
    println!("Number: {}",1);
    println!("{} is From: {}","Brad","Mass");

    //Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    //Named arguments
    println!("{name} likes to play {activity}", name = "John", activity="Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octo: {:o}", 10,10,10);

    //Placeholder for debug traits
    println!("{:?}",(12, true, "hello"));

    //Basic math
    println!("10 + 10 = {}", 10+10);
}