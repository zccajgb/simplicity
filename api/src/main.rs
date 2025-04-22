use dotenvy::dotenv_override;
use rocket_cors::Cors;
use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::users;
use services::fairing::AuthFairing;
use services::logger::TimestampLogger;

mod domain;
mod repos;
mod routes;
mod services;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    log::set_boxed_logger(Box::new(TimestampLogger))
        .inspect_err(|e| error!("Error setting logger {}", e))
        .unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let is_prod = std::env::var("PROD").unwrap_or_else(|_| "false".to_string());
    if is_prod != "true" {
        info!("Loading .env file");
        dotenv_override().ok();
    } else {
        info!("Running in production mode");
    }
    rocket::build()
        .mount("/", users::get_routes())
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
