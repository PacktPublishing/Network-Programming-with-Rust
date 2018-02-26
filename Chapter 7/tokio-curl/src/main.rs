extern crate curl;
extern crate tokio_core;
extern crate tokio_curl;

use curl::easy::Easy;
use tokio_core::reactor::Core;
use tokio_curl::Session;
use std::io::Write;
use std::fs::File;

fn main() {
    let mut core = Core::new().unwrap();
    let session = Session::new(core.handle());

    let mut handle = Easy::new();
    let mut file = File::create("foo.zip").unwrap();
    handle.get(true).unwrap();
    handle.url("http://ipv4.download.thinkbroadband.com/5MB.zip").unwrap();
    handle.header_function(|header| {
        print!("{}", std::str::from_utf8(header).unwrap());
        true
    }).unwrap();
    handle.write_function(move |data| {
        file.write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();

    let request = session.perform(handle);

    let mut response = core.run(request).unwrap();
    println!("{:?}", response.response_code());
}
