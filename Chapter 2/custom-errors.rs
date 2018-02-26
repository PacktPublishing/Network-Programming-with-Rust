use std::fmt;  
use std::error::Error;

#[derive(Debug)]
enum OperationsError {
    DivideByZeroError,
}

impl fmt::Display for OperationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
        OperationsError::DivideByZeroError => f.write_str("Cannot divide by zero"),
        }
    }
}

impl Error for OperationsError {
    fn description(&self) -> &str {
        match *self {
            OperationsError::DivideByZeroError => "Cannot divide by zero",
        }
    }
}

fn divide(dividend: u32, divisor: u32) -> Result<u32, OperationsError> {
    if divisor == 0u32 {
        Err(OperationsError::DivideByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let result1 = divide(100, 0);
    println!("{:?}", result1);

    let result2 = divide(100, 2);
    println!("{:?}", result2.unwrap());
}
