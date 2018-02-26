fn divide(dividend: u32, divisor: u32) -> Option<u32> {
    if divisor == 0u32 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main() {
    let result1 = divide(100, 0);

    match result1 {
        None => println!("Error occurred"),
        Some(result) => println!("The result is {}", result),
    }

    let result2 = divide(100, 2);
    println!("{:?}", result2.unwrap());
}
