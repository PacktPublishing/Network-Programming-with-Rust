extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;
use futures::{future, Future};
use tokio_proto::TcpServer;

// Codec implementation, our codec is a simple unit struct
pub struct CollatzCodec;

// Decoding a byte stream from the underlying socket
impl Decoder for CollatzCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        // Since a newline denotes end of input, read till a newline
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let line = buf.split_to(i);
            // and remove the newline
            buf.split_to(1);
            // try to decode into an UTF8 string before passing to the protocol
            match str::from_utf8(&line) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }
}

// Encoding a string to a newline terminated byte stream
impl Encoder for CollatzCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

// Protocol implementation as an unit struct
pub struct CollatzProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for CollatzProto {
    type Request = String;
    type Response = String;
    type Transport = Framed<T, CollatzCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(CollatzCodec))
    }
}

// Service implementation
pub struct CollatzService;

fn get_sequence(n: u64) -> Vec<u64> {
    let mut n = n.clone();
    let mut result = vec![];
    result.push(n);
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        result.push(n);
    }
    result
}

impl Service for CollatzService {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req.trim().parse::<u64>() {
            Ok(num) => {
                let res = get_sequence(num);
                Box::new(future::ok(format!("{:?}", res)))
            }
            Err(_) => Box::new(future::ok("Could not parse input as an u64".to_owned())),
        }
    }
}

fn main() {
    let addr = "0.0.0.0:9999".parse().unwrap();
    let server = TcpServer::new(CollatzProto, addr);
    server.serve(|| Ok(CollatzService));
}
