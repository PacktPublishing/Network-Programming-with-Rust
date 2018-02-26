#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;

use rocket_contrib::Template;
use rocket::{Rocket, State};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

struct VisitorCounter {
    visitor_number: AtomicUsize,
}

#[get("/webpage/<name>")]
fn webpage(name: String, visitor: State<VisitorCounter>) -> Template {
    let mut context = HashMap::new();
    context.insert("name", name);
    let current = visitor.visitor_number.fetch_add(1, Ordering::SeqCst);
    context.insert("visitor_number", current.to_string());
    Template::render("webpage", &context)
}

fn rocket() -> Rocket {
    rocket::ignite()
        .manage(VisitorCounter { visitor_number: AtomicUsize::new(1) })
        .mount("/", routes![webpage])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
