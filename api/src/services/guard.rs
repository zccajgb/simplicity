use super::auth::get_user_from_session_token;
use crate::routes::users::User;
use crate::services::auth::ApiKey;
use log::error;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cookie = request.cookies().get("session_token");
        let Some(cookie) = cookie else {
            // error!("No token found");
            return Outcome::Error((Status::Unauthorized, ()));
        };

        let session_token = cookie.value();
        let user = get_user_from_session_token(session_token).await;
        match user {
            Ok(user) => Outcome::Success(User::from_user_model(user)),
            Err(e) => {
                // error!("Error in guard: {:?}", e);
                Outcome::Error((Status::Unauthorized, ()))
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(api_key) = request.headers().get_one("X-Api-Key") else {
            error!("No API key found");
            return Outcome::Error((Status::Unauthorized, ()));
        };
        let api_key = ApiKey(api_key.to_string());
        match api_key.validate() {
            Ok(_) => Outcome::Success(api_key),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
