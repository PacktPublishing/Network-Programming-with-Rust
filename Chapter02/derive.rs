use std::fmt;
use std::fmt::Display;

#[derive(Debug, Hash)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> fmt::Display for Point<T> where T: Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p: Point<u32> = Point { x: 4u32, y: 2u32 };

    // uses Display
    println!("{}", p);

    // uses Debug
    println!("{:?}", p);
}
