use rocket::Route;
use rocket::http::Method::Get;
use rust_rocket_web::controllers::{top_controller, member_controller};

pub fn handler() -> Vec<Route> {
    // by using routes![ ... ]
    let mut type1 = routes![
        top_controller::login_page,
        top_controller::login_action,
        top_controller::test,
        top_controller::json2,
        member_controller::profile,
    ];

    // by using Route::new
    let type2 = vec! [
        Route::new(Get, "/", top_controller::index),
        Route::new(Get, "/view", top_controller::view),
        Route::new(Get, "/status/200", top_controller::status200),
        Route::new(Get, "/status/404", top_controller::status404),
        Route::new(Get, "/json", top_controller::json),

        Route::new(Get, "/member/", member_controller::index),
    ];

    // combine
    type1.extend(type2);
    type1
}
