pub fn fizz_main() {
    fizzbuzz_to(100);
}

fn is_divisible_by(dividendo: u16, divisor: u16) -> bool {
    if divisor == 0 {
        return false;
    }
    dividendo % divisor == 0
}

fn fizzbuzz(num: u16) -> () {
    if is_divisible_by(num, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(num, 3) {
        println!("fizz");
    } else if is_divisible_by(num, 5) {
        println!("buzz");
    } else {
        println!("{}", num)
    }
}

fn fizzbuzz_to(num: u16) {
    for num in 1..=num {
        fizzbuzz(num);
    }

}