use std::collections::HashMap;
use std::io::Cursor;
use rocket::{Request, Response};
use rocket::data::{Data};
use rocket::handler::Outcome;
use rocket::http::Status;
use rocket_contrib::templates::Template;
use serde::Serialize;
use rocket_contrib::json::Json;

pub fn index<'r>(request: &'r Request, _data: Data) -> Outcome<'r> {
    Outcome::from(request, "simple")
}

pub fn view<'r>(request: &'r Request, _data: Data) -> Outcome<'r> {
    let mut context = HashMap::new();
    context.insert("var", "123");

    let t = Template::render("index", &context);
    Outcome::from(request, t)
}

pub fn status404<'r>(request: &'r Request, _data: Data) -> Outcome<'r> {
    let response = Response::build()
        .status(Status::NotFound)
        .sized_body(Cursor::new("Brewing the best coffee!"))
        .finalize();

    Outcome::from(request, response)
}

pub fn status200<'r>(request: &'r Request, _data: Data) -> Outcome<'r> {
    let response = Response::build()
        .status(Status::Ok)
        .finalize();

    Outcome::from(request, response)
}

#[derive(Serialize)]
struct Task {
    name: String,
}

pub fn json<'r>(request: &'r Request, _data: Data) -> Outcome<'r> {
    let t = Task{ name: String::from("123") };
    Outcome::from(request, Json(&t))
}
