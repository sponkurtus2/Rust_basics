pub fn control_flow() {

    // If expression
    let number: i32 = 3;
    if (number <= 3) {
        println!("{} is less or equal to 3", number);
    } else {
        println!("{} is greater than 3", number);
    }

    // If inside a let statement
    let always_true: bool = true;
    let number2 = if always_true { 5 } else { 10 };
    println!("{}", number2);


    // Loop expression
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;
        println!("{}", counter);

        if (counter == 3) {
            break counter;
        }
    };
        println!("{}", counter);


    // While expression
    let mut while_number: i32 = 3;

    while while_number != 0 {
        println!("{}", while_number);
        while_number -= 1;
    }


    // Loop through a collection
    let number_collection: [i8; 5] = [1,2,3,4,5];

    for number in number_collection {
        println!("{}", number);
    }
}