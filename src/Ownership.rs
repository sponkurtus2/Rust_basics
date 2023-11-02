pub fn ownesrhip() {
    let vec1:Vec<i32> = vec![1, 4, 7];
    println!("{:?}", vec1);
    // Until this part of the code, we have a vector called vec1, which has some values
    // But if now, we create another vector, with the exact values as vec1, the ownership will happen
    // And those values will be transferred to the new vector, and will no longer be on the original vector
    // Here is an example
    let vec2: Vec<i32> = vec1;
    println!("{:?}", vec1);
    // If we run this code, we'll have an error, because vec1 no longer exist, and all of it's values are
    // stored in vec2
}