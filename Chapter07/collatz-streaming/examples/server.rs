extern crate collatz_streaming as collatz;
extern crate futures;

extern crate tokio_proto;

use tokio_proto::TcpServer;
use collatz::{CollatzProto, CollatzService};

fn main() {
    let addr = "0.0.0.0:9999".parse().unwrap();
    TcpServer::new(CollatzProto, addr).serve(|| Ok(CollatzService));
}
