use std::io;

fn input_data(prompt: &str) -> String{
    let mut prompt_str: String = String::new();
    println!("{}", prompt);

    io::stdin().read_line(&mut prompt_str).expect("Failed to read value");
    return prompt_str.trim().to_string();
}

pub fn input_from_user() {
    let name = input_data("What is your name");
    let age = input_data("And your age?");
    let age_int: u8 = age.parse().unwrap();

    println!("Name: {}, Age: {}", name, age_int);
}