extern crate mio;

use mio::*;
use mio::tcp::{TcpListener};

use std::net::SocketAddr;
use std::env;

const SERVER: Token = Token(0);

struct TCPServer {
    address: SocketAddr,
}

impl TCPServer {
    fn new(port: u32) -> Self {
        let address = format!("0.0.0.0:{}", port)
            .parse::<SocketAddr>().unwrap();

        TCPServer {
            address,
        }
    }

    fn run(&mut self) {
        let server = TcpListener::bind(&self.address)
            .expect("Could not bind to port");
        let poll = Poll::new().unwrap();
        poll.register(&server, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

        let mut events = Events::with_capacity(1024);
        loop {
            poll.poll(&mut events, None).unwrap();

            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        let (_stream, remote) = server.accept().unwrap();
                        println!("Connection from {}", remote);
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide only one port number as argument");
        std::process::exit(1);
    }
    let mut server = TCPServer::new(args[1].parse::<u32>().expect("Could not parse as u32"));
    server.run();
}
