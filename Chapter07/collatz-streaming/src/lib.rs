extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use futures::{future, Future, Poll, Stream};
use futures::sync::mpsc;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_core::reactor::Handle;
use tokio_proto::TcpClient;
use tokio_proto::streaming::{Body, Message};
use tokio_proto::streaming::pipeline::{ClientProto, Frame, ServerProto};
use tokio_proto::util::client_proxy::ClientProxy;
use tokio_service::Service;
use std::str::FromStr;
use bytes::{BufMut, BytesMut};
use std::{io, str};
use std::net::SocketAddr;

// Everything about clients
type CollatzMessage = Message<String, Body<String, io::Error>>;

#[derive(Debug)]
pub enum CollatzInput {
    Once(String),
    Stream(CollatzStream),
}

pub struct CollatzProto;

pub struct Client {
    inner: ClientProxy<CollatzMessage, CollatzMessage, io::Error>,
}

impl Client {
    pub fn connect(
        addr: &SocketAddr,
        handle: &Handle,
    ) -> Box<Future<Item = Client, Error = io::Error>> {
        let ret = TcpClient::new(CollatzProto)
            .connect(addr, handle)
            .map(|cp| Client { inner: cp });

        Box::new(ret)
    }
}

impl Service for Client {
    type Request = CollatzInput;
    type Response = CollatzInput;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = io::Error>>;

    fn call(&self, req: CollatzInput) -> Self::Future {
        Box::new(self.inner.call(req.into()).map(CollatzInput::from))
    }
}

// Everything about server
#[derive(Debug)]
pub struct CollatzStream {
    inner: Body<String, io::Error>,
}

impl CollatzStream {
    pub fn pair() -> (mpsc::Sender<Result<String, io::Error>>, CollatzStream) {
        let (tx, rx) = Body::pair();
        (tx, CollatzStream { inner: rx })
    }
}

impl Stream for CollatzStream {
    type Item = String;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<String>, io::Error> {
        self.inner.poll()
    }
}

pub struct CollatzCodec {
    decoding_head: bool,
}

impl From<CollatzMessage> for CollatzInput {
    fn from(src: CollatzMessage) -> CollatzInput {
        match src {
            Message::WithoutBody(line) => CollatzInput::Once(line),
            Message::WithBody(_, body) => CollatzInput::Stream(CollatzStream { inner: body }),
        }
    }
}

impl From<CollatzInput> for Message<String, Body<String, io::Error>> {
    fn from(src: CollatzInput) -> Self {
        match src {
            CollatzInput::Once(line) => Message::WithoutBody(line),
            CollatzInput::Stream(body) => {
                let CollatzStream { inner } = body;
                Message::WithBody("".to_string(), inner)
            }
        }
    }
}

impl Decoder for CollatzCodec {
    type Item = Frame<String, String, io::Error>;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, io::Error> {
        if let Some(n) = buf.as_ref().iter().position(|b| *b == b'\n') {
            let line = buf.split_to(n);

            buf.split_to(1);
            return match str::from_utf8(line.as_ref()) {
                Ok(s) => {
                    if s == "" {
                        let decoding_head = self.decoding_head;
                        self.decoding_head = !decoding_head;

                        if decoding_head {
                            Ok(Some(Frame::Message {
                                message: s.to_string(),
                                body: true,
                            }))
                        } else {
                            Ok(Some(Frame::Body { chunk: None }))
                        }
                    } else {
                        if self.decoding_head {
                            Ok(Some(Frame::Message {
                                message: s.to_string(),
                                body: false,
                            }))
                        } else {
                            Ok(Some(Frame::Body {
                                chunk: Some(s.to_string()),
                            }))
                        }
                    }
                }
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid string")),
            };
        }

        Ok(None)
    }
}

impl Encoder for CollatzCodec {
    type Item = Frame<String, String, io::Error>;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        match msg {
            Frame::Message { message, body } => {
                buf.reserve(message.len());
                buf.extend(message.as_bytes());
            }
            Frame::Body { chunk } => {
                if let Some(chunk) = chunk {
                    buf.reserve(chunk.len());
                    buf.extend(chunk.as_bytes());
                }
            }
            Frame::Error { error } => {
                return Err(error);
            }
        }
        buf.put_u8(b'\n');
        Ok(())
    }
}

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for CollatzProto {
    type Request = String;
    type RequestBody = String;
    type Response = String;
    type ResponseBody = String;
    type Error = io::Error;

    type Transport = Framed<T, CollatzCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let codec = CollatzCodec {
            decoding_head: true,
        };

        Ok(io.framed(codec))
    }
}

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for CollatzProto {
    type Request = String;
    type RequestBody = String;
    type Response = String;
    type ResponseBody = String;
    type Error = io::Error;

    type Transport = Framed<T, CollatzCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let codec = CollatzCodec {
            decoding_head: true,
        };

        Ok(io.framed(codec))
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

fn clean_line(line: &str) -> Result<u64, <u64 as FromStr>::Err> {
    line.trim().parse::<u64>()
}

impl Service for CollatzService {
    type Request = CollatzInput;
    type Response = CollatzInput;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req {
            CollatzInput::Once(line) => {
                println!("Server got: {}", line);
                let res = get_sequence(clean_line(&line).unwrap());
                Box::new(future::done(Ok(CollatzInput::Once(format!("{:?}", res)))))
            }
            CollatzInput::Stream(body) => {
                let resp = body.for_each(|line| {
                    println!("{}", line);
                    Ok(())
                }).map(|_| CollatzInput::Once("Foo".to_string()));

                Box::new(resp) as Box<Future<Item = Self::Response, Error = io::Error>>
            }
        }
    }
}
