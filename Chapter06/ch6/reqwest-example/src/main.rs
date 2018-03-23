extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

#[derive(Debug,Serialize, Deserialize)]
struct Post {
    title: String,
    body: String,
    pinned: bool,
}

fn main() {
    let url = "http://localhost:8000/posts";
    let post: Post = Post {title: "Testing this".to_string(), body: "Try to write something".to_string(), pinned: true};
    let client = reqwest::Client::new();
    let res = client.post(url)
            .json(&post)
            .send()
            .unwrap();
    println!("Got back: {}", res.status());

    let mut posts = client.get(url).send().unwrap();
    let json: Vec<Post> = posts.json().unwrap();
    for post in json {
        println!("{:?}", post);
    }
}
