use dotenv::dotenv;
use routes::projects;
use routes::scheduled;
use routes::tags;
use routes::tasks;
use routes::users;
use services::fairing::AuthFairing;
use rocket_cors::{AllowedOrigins, CorsOptions};

mod domain;
mod repos;
mod routes;
mod services;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/", tags::get_routes())
        .mount("/", tasks::get_routes())
        .mount("/", projects::get_routes())
        .mount("/", users::get_routes())
        .mount("/", scheduled::get_routes())
        .attach(make_cors())
        .attach(AuthFairing)
}

fn make_cors() -> CorsOptions {
    let allowed_origins = AllowedOrigins::some_exact(&["http://client:8080", "https://simplicity.buckleyresearch.com"]);
    let allowed_methods = vec![Method::Get, Method::Post, Method::Patch, Method::Delete]
        .into_iter()
        .map(From::from)
        .collect();
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS");
    cors
}