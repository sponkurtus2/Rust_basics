use std::io;
use rand::Rng;

// We have 2 kind of functions, the ones that return something, and the ones that don't
// Here is a function which takes an int as a parameter, and then prints the age
fn say_age(age: i32) { println!("Your age is {}", age) }

// Now, here is a function which returns something
fn random_numer() -> i32 {
    let mut rng = rand::thread_rng(); // We create the number generator

    // Now we generate the number
    let random_num = rng.gen_range(1..=5); // We generate a random number from 1 to 5
    return random_num;
}


pub fn rust_functions() {
    // We first need to create an instance of Stdin to interact with the standard output
    let mut user_input = String::new();

    println!("What is your age?");

    // Read the answer and save it
    io::stdin().read_line(&mut user_input).expect("Couldn't read the value");

    // Transform the value into an int
    let age: i32 = user_input.trim().parse().expect("Couldn't convert the number");

    say_age(age);
    println!("{}", random_numer());

}
