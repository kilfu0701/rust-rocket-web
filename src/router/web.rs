use rocket::{Request, Route, route};
use rocket::data::{Data};
use rocket::http::{Status, Method::{Get, Post}};
use rust_rocket_web::controllers::top_controller;

pub fn handler() -> Vec<Route> {
    //routes![top_controller::index]

    vec! [
        Route::new(Get, "/", top_controller::index),
        Route::new(Get, "/view", top_controller::view),
        Route::new(Get, "/status/200", top_controller::status200),
        Route::new(Get, "/status/404", top_controller::status404),
        Route::new(Get, "/json", top_controller::json),
    ]
}
