extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use std::net::SocketAddr;
use std::io::BufReader;
use futures::Future;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;

fn main() {
    let mut core = Core::new().expect("Could not create event loop");
    let handle = core.handle();
    let addr: SocketAddr = "127.0.0.1:9999".parse().expect("Could not parse as SocketAddr");
    let socket = TcpStream::connect(&addr, &handle);
    let request = socket.and_then(|socket| {
        tokio_io::io::write_all(socket, b"110\n")
    });
    let response = request.and_then(|(socket, _request)| {
        let sock = BufReader::new(socket);
        tokio_io::io::read_until(sock, b'\n', Vec::new())
    });
    let (_socket, data) = core.run(response).unwrap();
    println!("{}", String::from_utf8_lossy(&data));
}
