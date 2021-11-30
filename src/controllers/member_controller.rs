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


pub fn index<'r>(request: &'r Request, _data: Data) -> Outcome<'r> {
    Outcome::from(request, "simple")
}
