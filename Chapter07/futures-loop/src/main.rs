extern crate futures;
extern crate tokio_core;

use std::io;
use std::io::BufRead;
use futures::Future;
use tokio_core::reactor::Core;

fn check_prime_boxed(n: u64) -> Box<Future<Item = bool, Error = io::Error>> {
    for i in 2..n {
        if n % i == 0 {
            return Box::new(futures::future::ok(false));
        }
    }
    Box::new(futures::future::ok(true))
}

fn main() {
    let mut core = Core::new().expect("Could not create event loop");
    let stdin = io::stdin();

    loop {
        let mut line = String::new();
        stdin
            .lock()
            .read_line(&mut line)
            .expect("Could not read from stdin");
        let input = line.trim()
            .parse::<u64>()
            .expect("Could not parse input as u64");
        let result = core.run(check_prime_boxed(input))
            .expect("Could not run future");
        println!("{}", result);
    }
}
