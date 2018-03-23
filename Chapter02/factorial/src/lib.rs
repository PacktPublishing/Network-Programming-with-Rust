#[allow(unused_macros)]

/// The factorial crate provides a macro to compute factorial of a given
/// integer
/// # Examples
///
/// ```
/// # #[macro_use] extern crate factorial;
/// # fn main() {
/// assert_eq!(factorial!(0), 1);
/// assert_eq!(factorial!(6), 720);
/// # }
/// ```
#[macro_export]
macro_rules! factorial {
    ($x:expr) => {
        {
            let mut result = 1;
            for i in 1..($x+1) {
                result = result * i;
            }
            result
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_factorial() {
        assert_eq!(factorial!(5), 120);
    }
}
