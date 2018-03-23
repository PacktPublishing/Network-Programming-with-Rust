extern crate futures;
extern crate hyper;
extern crate native_tls;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_tls;

use std::io;
use std::{thread, time};
use futures::future::{ok, Future};
use hyper::server::Http;
use hyper::header::ContentLength;
use hyper::{Request, Response, StatusCode};
use native_tls::{Pkcs12, TlsAcceptor};
use tokio_proto::TcpServer;
use tokio_service::Service;
use tokio_tls::proto;

fn heavy_work() -> String {
    let duration = time::Duration::from_millis(100);
    thread::sleep(duration);
    "done".to_string()
}

struct SlowMo;

impl Service for SlowMo {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = Box<Future<Item = Response, Error = io::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let b = heavy_work().into_bytes();
        println!("Request: {:?}", req);
        Box::new(ok(Response::new()
            .with_status(StatusCode::Ok)
            .with_header(ContentLength(b.len() as u64))
            .with_body(b)))
    }
}

fn main() {
    let raw_cert = include_bytes!("../cert.pfx");
    let cert = Pkcs12::from_der(raw_cert, "foobar").unwrap();
    let acceptor = TlsAcceptor::builder(cert).unwrap().build().unwrap();
    let proto = proto::Server::new(Http::new(), acceptor);
    let addr = "0.0.0.0:9999".parse().unwrap();
    let srv = TcpServer::new(proto, addr);
    println!("Listening on {}", addr);
    srv.serve(|| Ok(SlowMo));
}
