use dotenv::dotenv;
use routes::projects;
use routes::tags;
use routes::tasks;
use services::fairing::AuthFairing;

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
        .attach(AuthFairing)
}
