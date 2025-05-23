use super::auth::{get_user_from_session_token, validate_token};
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
        if request.uri().path().contains("/login") {
            return;
        }
        if request.uri().path().contains("/dailyupdate") {
            return;
        }

        let cookie = request.cookies().get("session_token");
        let Some(cookie) = cookie else {
            // error!("Error in fairing, No token found");
            request.local_cache(|| Status::Unauthorized);
            return;
        };

        let session_token = cookie.value();
        if let Err(e) = get_user_from_session_token(session_token).await {
            // error!("Error in fairing, Invalid token. {:?}", e);
            request.local_cache(|| Status::Unauthorized);
            return;
        };
    }
}
