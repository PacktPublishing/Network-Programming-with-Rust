use super::schema::posts;
use rocket::{Request, Data};
use rocket::data::{FromDataSimple, Outcome};
use rocket::http::{Status, ContentType};
use rocket::Outcome::*;
use serde_json;
use std::io::Read;

#[derive(Queryable)]
#[derive(Serialize,Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub pinned: bool,
}

#[derive(Insertable, Deserialize, AsChangeset, Debug, Default)]
#[table_name="posts"]
pub struct PostData {
    pub title: String,
    pub body: String,
    pub pinned: bool,
}

impl FromDataSimple for PostData {
    type Error = String;

    #[allow(unused_variables)]
    fn from_data(req: &Request, data: Data) -> Outcome<Self, String> {
        let person_ct = ContentType::new("application", "json");
        if req.content_type() != Some(&person_ct) {
            return Outcome::Forward(data);
        }
        let mut buffer = String::new();
        let reader = data.open().read_to_string(&mut buffer);

        match serde_json::from_str::<Self>(&buffer) {
            Ok(pd) => Success(pd),
            Err(e) => Failure((Status::BadRequest, e.to_string())),
        }
    }
}
