extern crate futures;
extern crate rand;

use std::{io, thread};
use std::time::Duration;
use futures::stream::Stream;
use futures::{Poll, Async};
use rand::{thread_rng, Rng};
use futures::Future;

#[derive(Debug)]
struct CollatzStream {
    current: u64,
    end: u64,
}

impl CollatzStream {
    fn new(start: u64) -> CollatzStream {
        CollatzStream {
            current: start,
            end: 1
        }
    }
}

impl Stream for CollatzStream {
    type Item = u64;
    type Error = io::Error;
    fn poll(&mut self) -> Poll<Option<Self::Item>, io::Error> {
        let d = thread_rng().gen_range::<u64>(1, 5);
        thread::sleep(Duration::from_secs(d));
        if self.current % 2 == 0 {
            self.current = self.current / 2; 
        } else {
            self.current = 3 * self.current + 1;
        }
        if self.current == self.end {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::Ready(Some(self.current)))
        }
    }
}

fn main() {
    let stream = CollatzStream::new(10);
    let f = stream.for_each(|num| {
        println!("{}", num);
        Ok(())
    });
    f.wait().ok();
}