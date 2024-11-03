use super::auth::{validate_token_and_get_user, User};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");

        let Some(token) = token else {
            return Outcome::Error((Status::Unauthorized, ()));
        };

        return match validate_token_and_get_user(token) {
            Ok(user) => Outcome::Success(user),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        };
    }
}
