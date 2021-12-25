use std::collections::HashMap;
use std::io::Cursor;
use rocket::{Request, Response, Responder};
use rocket::data::{Data};
use rocket::handler::Outcome;
use rocket::http::Status;
use rocket::response::content;
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;
use serde::Serialize;
use diesel::result::Error;

use crate::db::Connection;
use crate::models::user::User;
use crate::repositories::user_repository;

pub fn index<'r>(request: &'r Request, _data: Data) -> Outcome<'r> {
    Outcome::from(request, "simple")
}

// login page
#[get("/member/<id>")]
pub fn profile(id: u64, connection: Connection) -> Template {
    user_repository::get_user(id, &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error));

    let mut context = HashMap::new();
    context.insert("var", "123");

    Template::render("login", &context)
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}
