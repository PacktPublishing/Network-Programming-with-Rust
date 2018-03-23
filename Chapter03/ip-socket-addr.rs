#![feature(ip)]

use std::net::{IpAddr, SocketAddr};

fn main() {
    let local: IpAddr = "127.0.0.1".parse().unwrap();
    assert!(local.is_loopback());

    let global: IpAddr = IpAddr::from([0, 0, 0x1c9, 0, 0, 0xafc8, 0, 0x1]);
    assert!(global.is_global());

    let local_sa: SocketAddr = "127.0.0.1:80".parse().unwrap();
    assert!(local_sa.is_ipv4());

    let global_sa = SocketAddr::new(global, 80u16);
    assert!(global_sa.is_ipv6());
}
