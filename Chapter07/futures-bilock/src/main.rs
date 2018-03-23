extern crate futures;
extern crate rand;

use std::thread;
use std::fmt::Debug;
use std::time::Duration;
use futures::{Future, Async};
use rand::{thread_rng, Rng};

use futures::sync::{mpsc, BiLock};
use futures::{Sink, Stream};
use futures::sync::mpsc::Receiver;

fn sender(send: &BiLock<u64>) -> &'static str {
    match send.poll_lock() {
        Async::Ready(mut lock) => *lock += 1,
        Async::NotReady => ()
    }
    let mut d = thread_rng();
    thread::sleep(Duration::from_secs(d.gen_range::<u64>(1, 5)));
    d.choose(&["ping", "pong"]).unwrap()
}

fn receiver<T: Debug>(recv: Receiver<T>, recv_lock: BiLock<u64>) {
    match recv_lock.poll_lock() {
        Async::Ready(lock) => println!("Value of lock {}", *lock),
        Async::NotReady => ()
    }
    let f = recv.for_each(|item| {
        println!("{:?}", item);
        Ok(())
    });
    f.wait().ok();
}

fn main() {
    let counter = 0;
    let (send, recv) = BiLock::new(counter);
    let (tx, rx) = mpsc::channel(100);
    let h1 = thread::spawn(move || {
        tx.send(sender(&send)).wait().ok();
    });
    let h2 = thread::spawn(|| {
        receiver::<&str>(rx, recv);
    });
    h1.join().unwrap();
    h2.join().unwrap();
}
