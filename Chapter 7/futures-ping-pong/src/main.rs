extern crate futures;
extern crate rand;
extern crate tokio_core;

use std::thread;
use std::fmt::Debug;
use std::time::Duration;
use futures::Future;
use rand::{thread_rng, Rng};

use futures::sync::mpsc;
use futures::{Sink, Stream};
use futures::sync::mpsc::Receiver;

fn sender() -> &'static str {
    let mut d = thread_rng();
    thread::sleep(Duration::from_secs(d.gen_range::<u64>(1, 5)));
    d.choose(&["ping", "pong"]).unwrap()
}

fn receiver<T: Debug>(recv: Receiver<T>) {
    let f = recv.for_each(|item| {
        println!("{:?}", item);
        Ok(())
    });
    f.wait().ok();
}

fn main() {
    let (tx, rx) = mpsc::channel(100);
    let h1 = thread::spawn(|| {
        tx.send(sender()).wait().ok();
    });
    let h2 = thread::spawn(|| {
        receiver::<&str>(rx);
    });
    h1.join().unwrap();
    h2.join().unwrap();
}
