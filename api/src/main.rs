use dotenv::dotenv;
use routes::tasks::get_routes;
use services::fairing::AuthFairing;

mod repos;
mod routes;
mod services;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", get_routes()).attach(AuthFairing)
}
