use std::{env, str};
use std::net::{UdpSocket, Ipv4Addr};

fn main() {
    let mcast_group: Ipv4Addr = "239.0.0.1".parse().unwrap();
    let port: u16 = 6000;
    let any = "0.0.0.0".parse().unwrap();
    let mut buffer = [0u8; 1600];
    if env::args().count() > 1 {
        let socket = UdpSocket::bind((any, port)).expect("Could not bind client socket");
        socket.join_multicast_v4(&mcast_group, &any).expect("Could not join multicast group");
        socket.recv_from(&mut buffer).expect("Failed to write to server");
        print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
    } else {
        let socket = UdpSocket::bind((any, 0)).expect("Could not bind socket");
        socket.send_to("Hello world!".as_bytes(), &(mcast_group, port)).expect("Failed to write data");
    }
}
