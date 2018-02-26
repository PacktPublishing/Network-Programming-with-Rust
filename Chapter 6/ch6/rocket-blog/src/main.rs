#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;

mod schema;
mod db;
mod post;
mod models;
mod error;

use db::DB;
use post::{get_posts, get_post, create_post, delete_post, update_post};
use models::*;
use rocket_contrib::Json;
use rocket::response::status::{Created, NoContent};
use rocket::Rocket;
use error::ApiError;

#[get("/posts", format = "application/json")]
fn posts_get(db: DB) -> Result<Json<Vec<Post>>, ApiError> {
    let posts = get_posts(&db)?;
    Ok(Json(posts))
}

#[get("/posts/<id>", format = "application/json")]
fn post_get(db: DB, id: i32) -> Result<Json<Post>, ApiError> {
    let post = get_post(&db, id)?;
    Ok(Json(post))
}

#[post("/posts", format = "application/json", data = "<post>")]
fn post_create(db: DB, post: PostData) -> Result<Created<String>, ApiError> {
    let post = create_post(&db, post);
    let url = format!("/post/{}", post);
    Ok(Created(url, Some("Done".to_string())))
}

#[patch("/posts/<id>", format = "application/json", data = "<post>")]
fn post_edit(db: DB, id: i32, post: PostData) -> Result<Json<bool>, ApiError> {
    let post = update_post(&db, id, post);
    Ok(Json(post))
}

#[delete("/posts/<id>")]
fn post_delete(db: DB, id: i32) -> Result<NoContent, ApiError> {
    delete_post(&db, id)?;
    Ok(NoContent)
}

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![post_create, posts_get, post_delete, post_edit, post_get])
}

fn main() {
        rocket().launch();
}
