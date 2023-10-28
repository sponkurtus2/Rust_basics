pub fn data_types() {
    // Integer types //
    /*
    *   Length      Signed      Unsigned
    *   8-Bit       i8          u8
    *   16-Bit      i16         u16
    *   32-Bit      i32         u32
    *   64-Bit      i64         u64
    *   128-Bit     i128        u128
    */
    /*
    Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses.
    So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127.
    Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.
    */

    // Floating point types //
    let num1: f32 = 32.00; // Has less precision
    let num2: f64 = 64.00; // Has more precision

    println!("Num with less precision: {}, and num with more precision: {}", num1, num2);


    // Boolean types //
    let True: bool = true;
    let False: bool = false;

    // Character type //
    // In rust, '' the single quotes are for declaring characters
    let c: char = 'z';
    let emoji: char = '🐵';
    println!("Monki: {}", emoji);


    // Tuples //
    let tuple: (i8, i16, f32) = (1,20, 45.30);

    // We have 2 ways to acces to the tuple values, the first one is like this:
    let (x, y, z) = tuple;
    println!("Tuple \n Value 1 : {} \n Value 2: {} \n Value 3: {} ", x, y, z);

    // The second one is using a period .
    let tuple_with_period: (i8, i8, i8) = (5, 6, 7);
    println!("First value of the tuple with period: {}", tuple_with_period.0);


    // Array //
    let names: [&str; 3] = ["Carlos", "Miriam", "Mariana"];
    println!("First element of the array names: {}", names[2]);

}