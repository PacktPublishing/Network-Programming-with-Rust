#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn blast_off() -> &'static str {
    "Hello, Rocket!"
}

fn main() {
    rocket::ignite().mount("/", routes![blast_off]).launch();
}
