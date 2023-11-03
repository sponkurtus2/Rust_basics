pub fn generic_types() {
    // This code will return the largest value in any number - char by using generic types <T>
    let numbers: Vec<i64> = vec![1020319293, 1120930810833131393, 11291919293, 22929292929191, 129319292, 112210293, 102999];
    println!("{}", find_largest(numbers));

    let characters: Vec<char> = vec!['a', 'b', 'c', 'z', 'x', 'y'];
    println!("{}", find_largest(characters));

    let decimal_numbers: Vec<f32> = vec![19.20, 2939.23, 29291.10];
    println!("{}", find_largest(decimal_numbers));
}

fn find_largest<T: Copy + PartialOrd>(data_list: Vec<T>) -> T {
    let mut largest = data_list[0];
    for data in data_list {
        if data > largest {
            largest = data;
        }
    }
    largest
}