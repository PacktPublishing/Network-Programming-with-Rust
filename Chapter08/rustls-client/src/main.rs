use std::sync::Arc;

use std::net::TcpStream;
use std::io::{Read, Write};

extern crate rustls;
extern crate webpki;
extern crate webpki_roots;

fn main() {
    let mut tls = rustls::ClientConfig::new();
    tls.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

    let name = webpki::DNSNameRef::try_from_ascii_str("my.domain.io").expect("Could not resolve name");
    let mut sess = rustls::ClientSession::new(&Arc::new(tls), name);
    let mut conn = TcpStream::connect("my.domain.io:8000").unwrap();
    let mut stream = rustls::Stream::new(&mut sess, &mut conn);
    stream.write(concat!("GET / HTTP/1.1\r\n",
                      "Connection: close\r\n",
                      "\r\n")
              .as_bytes())
        .expect("Could not write request");
    let mut plaintext = Vec::new();
    stream.read_to_end(&mut plaintext).expect("Could not read");
    println!("{}", String::from_utf8(plaintext).expect("Could not print output"));
}