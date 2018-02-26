extern crate collatz_streaming as collatz;

extern crate futures;
extern crate tokio_core;
extern crate tokio_service;

use collatz::{CollatzInput, CollatzStream};
use std::thread;
use futures::Sink;
use futures::Future;
use tokio_core::reactor::Core;
use tokio_service::Service;

pub fn main() {
    let addr = "127.0.0.1:9999".parse().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    core.run(
        collatz::Client::connect(&addr, &handle)
            .and_then(|client| {
                client.call(CollatzInput::Once("10".to_string()))
                    .and_then(move |response| {
                        println!("Response: {:?}", response);

                        let (mut tx, rx) = CollatzStream::pair();

                        thread::spawn(move || {
                            for msg in &["Hello", "world", "!"] {
                                tx = tx.send(Ok(msg.to_string())).wait().unwrap();
                            }
                        });

                        client.call(CollatzInput::Stream(rx))
                    })
                    .and_then(|response| {
                        println!("Response: {:?}", response);
                        Ok(())
                    })
            })
    ).unwrap();
}
