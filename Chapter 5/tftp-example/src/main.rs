extern crate tftp_server;

use tftp_server::server::TftpServer;

use std::net::SocketAddr;
use std::str::FromStr;

fn main() {
    let addr = format!("0.0.0.0:{}", 69);
    let socket_addr = SocketAddr::from_str(addr.as_str()).expect("Error parsing address");
    let mut server = TftpServer::new_from_addr(&socket_addr).expect("Error creating server");
    match server.run() {
        Ok(_) => println!("Server completed successfully!"),
        Err(e) => println!("Error: {:?}", e),
    }
}
