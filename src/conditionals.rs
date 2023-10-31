use std::io;
use std::process::exit;

pub fn conditionals() {
    /*println!("What is your age? ");
    let mut age: String = String::new();
    io::stdin().read_line(&mut age).unwrap();

    let mut age_int: u8 = age.trim().parse().unwrap();

    if age_int < 18 {
        println!("You must be 18+");
    } else {
        println!("Welcome :D")
    }*/

    /*
    User will write if he wants to take the blue or red pill
    Blue pill -> Matrix
    Red pill -> Real world
    Not valid option -> Expect error ( Matrix has explode )
    */
    println!("Hi, do you want the blue or red pill? ");
    let mut pill: String = String::new();
    io::stdin().read_line(&mut pill).unwrap().to_string();

    if pill == "blue" {
        println!("Welcome to the matrix...")
    } else if pill == "red" {
        println!("Welcome back to the real world");
    } else {
    println!("Not valid pill...")
    }
}