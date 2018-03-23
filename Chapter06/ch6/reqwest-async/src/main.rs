extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate futures;
extern crate tokio_core;

use futures::Future;
use tokio_core::reactor::Core;
use reqwest::unstable::async::{Client, Decoder};
use std::mem;
use std::io::{self, Cursor};
use futures::Stream;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    title: String,
    body: String,
    pinned: bool,
}

fn main() {
    let mut core = Core::new().expect("Could not create core");
    let url = "http://localhost:8000/posts";
    let post: Post = Post {
        title: "Testing this".to_string(),
        body: "Try to write something".to_string(),
        pinned: true,
    };
    let client = Client::new(&core.handle());
    let res = client.post(url).json(&post).send().and_then(|mut res| {
        println!("{}", res.status());
        Ok(())
    });
    core.run(res).unwrap();

    let mut posts = client
        .get(url)
        .send()
        .and_then(|mut res| {
            println!("{}", res.status());
            let body = mem::replace(res.body_mut(), Decoder::empty());
            body.concat2().map_err(Into::into)
        })
        .and_then(|body| {
            let mut body = Cursor::new(body);
            let mut writer: Vec<u8> = vec![];
            io::copy(&mut body, &mut writer).unwrap();
            let posts: Vec<Post> = serde_json::from_str(std::str::from_utf8(&writer).unwrap())
                .unwrap();
            for post in posts {
                println!("{:?}", post);
            }
            Ok(())
        });
    core.run(posts).unwrap();
}
