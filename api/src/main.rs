use dotenv::dotenv;
use rocket_cors::Cors;
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::projects;
use routes::scheduled;
use routes::tags;
use routes::tasks;
use routes::users;
use services::fairing::AuthFairing;

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

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8080",
        "https://localhost:8080",
        "https://127.0.0.1:8080",
        "https://simplicity.buckleyresearch.com",
    ]);
    let allowed_origins = AllowedOrigins::all();

    let cors = CorsOptions {
        allowed_origins,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS");
    cors
}
