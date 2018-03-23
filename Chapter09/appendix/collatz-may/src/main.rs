#![feature(conservative_impl_trait)]

#[macro_use]
extern crate generator;
#[macro_use]
extern crate may;

use std::env;
use generator::Gn;

fn collatz_generator(start: u64) -> impl Iterator<Item = u64> {
    Gn::new_scoped(move |mut s| {
        let end = 1u64;
        let mut current: u64 = start;
        while current != end {
            s.yield_(current);
            if current % 2 == 0 {
                current /= 2;
            } else {
                current = 3 * current + 1;
            }
        }
        s.yield_(end);
        done!();
    })
}

fn collatz(start: u64) -> Vec<u64> {
    let end = 1u64;
    let mut current: u64 = start;
    let mut result = Vec::new();
    while current != end {
        result.push(current);
        if current % 2 == 0 {
            current /= 2;
        } else {
            current = 3 * current + 1;
        }
    }
    result.push(end);
    result
}

fn main() {
    let input = env::args()
        .nth(1)
        .expect("Please provide only one argument")
        .parse::<u64>()
        .expect("Could not convert input to integer");
    go!(move || {
        println!("{:?}", collatz(input));
    }).join()
        .unwrap();

    let results = collatz_generator(input);
    for result in results {
        println!("{}", result);
    }
}
