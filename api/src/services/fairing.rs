use super::auth::validate_token;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::request::Request;

pub struct AuthFairing;

#[rocket::async_trait]
impl Fairing for AuthFairing {
    fn info(&self) -> Info {
        Info {
            name: "Authentication Fairing",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut rocket::Data<'_>) {
        if request.uri().path().contains("/public") {
            return;
        }
        let Some(auth_token) = request.headers().get_one("Authorization") else {
            request.local_cache(|| Status::Unauthorized);
            return;
        };
        let Ok(_) = validate_token(auth_token) else {
            request.local_cache(|| Status::Unauthorized);
            return;
        };
    }
}
