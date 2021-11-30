#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[macro_use] extern crate rocket;

mod router;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())                    // init template engine
        .mount("/", router::web::handler())             // top group routing
        .mount("/public", StaticFiles::from("public"))  // serve local static files
        .launch();
}
