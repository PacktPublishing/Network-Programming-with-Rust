extern crate may;
extern crate may_minihttp;
extern crate num_cpus;

use std::io;
use may_minihttp::{HttpServer, HttpService, Request, Response};
use std::{thread, time};

fn heavy_work() -> String {
    let duration = time::Duration::from_millis(100);
    thread::sleep(duration);
    "done".to_string()
}

#[derive(Clone, Copy)]
struct Echo;

impl HttpService for Echo {
    fn call(&self, req: Request) -> io::Result<Response> {
        println!("Incoming request {:?}", req);
        let mut resp = Response::new();
        match (req.method(), req.path()) {
            ("GET", "/data") => {
                let b = heavy_work();
                resp.body(&b).status_code(200, "OK");
            }
            (&_, _) => {
                resp.status_code(404, "Not found");
            }
        }
        Ok(resp)
    }
}

fn main() {
    may::config().set_io_workers(num_cpus::get());
    let server = HttpServer(Echo).start("0.0.0.0:3000").unwrap();
    server.join().unwrap();
}
