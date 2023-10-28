pub fn variables_and_mutability() {
    // Here we are declaring a number which is NOT mutable, it means, it's value CANNOT change
    let number: i16 = 5;
    println!("The not mutable number is {}", number);

    // If we want to make a variable mutable, we can do it with the following code
    let mut mutable_number: i16 = 5;
    println!("The mutable number is {}", mutable_number);
    mutable_number = 6;
    println!("The new mutable number is {}", mutable_number);

    // CONSTANTS
    const PI: f32 = 3.1316;
    println!("PI equals: {}", PI);

}