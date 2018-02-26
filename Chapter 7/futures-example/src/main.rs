#![feature(conservative_impl_trait)]
extern crate futures;
extern crate futures_cpupool;

use std::io;
use futures::Future;
use futures_cpupool::CpuPool;

fn check_prime_boxed(n: u64) -> Box<Future<Item = bool, Error = io::Error>> {
    for i in 2..n {
        if n % i == 0 { return Box::new(futures::future::ok(false)); }
    }
    Box::new(futures::future::ok(true))
}

fn check_prime_impl_trait(n: u64) -> impl Future<Item = bool, Error = io::Error> {
        for i in 2..n {
        if n % i == 0 { return futures::future::ok(false); }
    }
    futures::future::ok(true)
}

fn check_prime(n: u64) -> bool {
    for i in 2..n {
        if n % i == 0 { return false }
    }
    true
}

fn main() {
    let input: u64 = 58466453;
    println!("Right before first call");
    let res_one = check_prime_boxed(input);
    println!("Called check_prime_boxed");
    let res_two = check_prime_impl_trait(input);
    let r = futures::executor::spawn(res_one);
    println!("Called check_prime_impl_trait");
    println!("Results are {} and {}", res_one.wait().unwrap(), res_two.wait().unwrap());

    let thread_pool = CpuPool::new(4);
    let res_three = thread_pool.spawn_fn(move || {
        let temp = check_prime(input);
        let result: Result<bool, ()> = Ok(temp);
        result
    });
    println!("Called check_prime in another thread");
    println!("Result from the last call: {}", res_three.wait().unwrap());
}
