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
use rocket::request::Form;

pub fn index<'r>(request: &'r Request, _data: Data) -> Outcome<'r> {
    Outcome::from(request, "simple")
}

// login page
#[get("/login")]
pub fn login_page() -> Template {
    let mut context = HashMap::new();
    context.insert("var", "123");

    Template::render("login", &context)
}


#[derive(FromForm, Serialize)]
pub struct LoginData {
    username: String,
    password: String,
}
// do login action
#[post("/login", data = "<login_data>")]
pub fn login_action(login_data: Form<LoginData>) -> Template {
    let context = HashMap::from([
        ("username", &*login_data.username),
        ("password", &*login_data.password),
    ]);

    Template::render("login_receive", &context)
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

#[derive(Responder, Serialize, Debug)]
#[response(status = 200, content_type = "json")]
pub struct Task {
    name: String,
}

pub fn json<'r>(request: &'r Request, _data: Data) -> Outcome<'r> {
    let t = Task{ name: String::from("123") };
    Outcome::from(request, Json(&t))
}

#[get("/test")]
pub fn test() -> Json<Task> {
    Json(Task{ name: String::from("1234") })
}

#[get("/json2")]
pub fn json2() -> content::Json<&'static str> {
    content::Json(r#"{ "hi": "world" }"#)
}
