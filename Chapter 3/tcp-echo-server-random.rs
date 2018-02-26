use std::net::{TcpListener, TcpStream};
use std::thread;
use std::rand::thread_rng;

use std::io::{Read, Write, Error};

fn flip() -> bool {
    let choices = [true, false];
    let mut rng = thread_rng();
    rng.choose(&choices).ok()
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(()) }
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
