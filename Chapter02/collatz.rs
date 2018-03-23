struct Collatz {
    current: u64,
    end: u64,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.current % 2 == 0 {
            self.current = self.current / 2;
        } else {
            self.current = 3 * self.current + 1;
        }

        if self.current == self.end {
            None
        } else {
            Some(self.current)
        }
    }
}

fn collatz(start: u64) -> Collatz {
    Collatz { current: start, end: 1u64 }
}

fn main() {
    let input = 10;

    // First 2 items
    for n in collatz(input).take(2) {
        println!("{}", n);
    }

    // Dropping first 2 items
    for n in collatz(input).skip(2) {
        println!("{}", n);
    }
}
