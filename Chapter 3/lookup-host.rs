#![feature(lookup_host)]

use std::env;
use std::net::lookup_host;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide only one host name");
        std::process::exit(1);
    } else {
        let addresses = lookup_host(&args[1]).unwrap();
        for address in addresses {
            println!("{}", address.ip());
        }
    }
}
