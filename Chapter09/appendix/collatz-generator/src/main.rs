#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::env;

fn main() {
    let input = env::args()
        .nth(1)
        .expect("Please provide only one argument")
        .parse::<u64>()
        .expect("Could not convert input to integer");

    let mut generator = || {
        let end = 1u64;
        let mut current: u64 = input;
        while current != end {
            yield current;
            if current % 2 == 0 {
                current /= 2;
            } else {
                current = 3 * current + 1;
            }
        }
        return end;
    };

    loop {
        match generator.resume() {
            GeneratorState::Yielded(el) => println!("{}", el),
            GeneratorState::Complete(el) => {
                println!("{}", el);
                break;
            }
        }
    }
}
