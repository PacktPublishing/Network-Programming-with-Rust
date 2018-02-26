extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use futures::{future, Future};

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::TcpClient;
use tokio_proto::multiplex::{ClientProto, ClientService, RequestId, ServerProto};
use tokio_service::Service;

use bytes::{BigEndian, Buf, BufMut, BytesMut};

use std::{io, str};
use std::net::SocketAddr;

// Everything client side
pub struct Client {
    inner: ClientService<TcpStream, CollatzProto>,
}

impl Client {
    pub fn connect(
        addr: &SocketAddr,
        handle: &Handle,
    ) -> Box<Future<Item = Client, Error = io::Error>> {
        let ret = TcpClient::new(CollatzProto)
            .connect(addr, handle)
            .map(|service| Client {
                inner: service,
            });

        Box::new(ret)
    }
}

impl Service for Client {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = Box<Future<Item = String, Error = io::Error>>;

    fn call(&self, req: String) -> Self::Future {
        Box::new(self.inner.call(req).and_then(move |resp| Ok(resp)))
    }
}

// Everything server side
pub struct CollatzCodec;
pub struct CollatzProto;
type CollatzFrame = (RequestId, String);

impl Decoder for CollatzCodec {
    type Item = CollatzFrame;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<CollatzFrame>, io::Error> {
        if buf.len() < 5 {
            return Ok(None);
        }

        let newline = buf[4..].iter().position(|b| *b == b'\n');
        if let Some(n) = newline {
            let line = buf.split_to(n + 4);
            buf.split_to(1);
            let request_id = io::Cursor::new(&line[0..4]).get_u32::<BigEndian>();
            return match str::from_utf8(&line.as_ref()[4..]) {
                Ok(s) => Ok(Some((u64::from(request_id), s.to_string()))),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
            };
        }

        Ok(None)
    }
}

impl Encoder for CollatzCodec {
    type Item = CollatzFrame;
    type Error = io::Error;

    fn encode(&mut self, msg: CollatzFrame, buf: &mut BytesMut) -> io::Result<()> {
        let len = 4 + msg.1.len() + 1;
        buf.reserve(len);

        let (request_id, msg) = msg;

        buf.put_u32::<BigEndian>(request_id as u32);
        buf.put_slice(msg.as_bytes());
        buf.put_u8(b'\n');

        Ok(())
    }
}

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for CollatzProto {
    type Request = String;
    type Response = String;

    type Transport = Framed<T, CollatzCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(CollatzCodec))
    }
}

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for CollatzProto {
    type Request = String;
    type Response = String;

    type Transport = Framed<T, CollatzCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(CollatzCodec))
    }
}

pub struct CollatzService;

fn get_sequence(mut n: u64) -> Vec<u64> {
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
