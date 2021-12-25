#![feature(proc_macro_hygiene, decl_macro)]
use dotenv::dotenv;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[macro_use] extern crate rocket;
#[macro_use] extern crate dotenv;
#[macro_use] extern crate diesel;

mod router;
mod db;
mod repositories;
mod models;
mod schema;

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(Template::fairing())                    // init template engine
        .manage(db::connect())                    // init template engine
        .mount("/", router::web::handler())             // top group routing
        .mount("/public", StaticFiles::from("public"))  // serve local static files
        .launch();
}
