extern crate ipnetwork;

use std::net::Ipv4Addr;
use ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};

fn main() {
    let net = IpNetwork::new("192.168.122.0".parse().unwrap(), 22).expect("Could not construct a network");
    let str_net: IpNetwork = "192.168.122.0/22".parse().unwrap();

    assert!(net == str_net);
    assert!(net.is_ipv4());

    let net4: Ipv4Network = "192.168.121.0/22".parse().unwrap();
    assert!(net4.size() == 2u64.pow(32 - 22));
    assert!(net4.contains(Ipv4Addr::new(192, 168, 121, 3)));

    let _net6: Ipv6Network = "2001:db8::0/96".parse().unwrap();
    for addr in net4.iter().take(10) {
        println!("{}", addr);
    }
}
