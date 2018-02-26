extern crate collatz_multiplexed as collatz;

extern crate tokio_proto;

use tokio_proto::TcpServer;

use collatz::{CollatzService, CollatzProto};

fn main() {
    let addr = "0.0.0.0:9999".parse().unwrap();
    TcpServer::new(CollatzProto, addr).serve(|| Ok(CollatzService));
}
