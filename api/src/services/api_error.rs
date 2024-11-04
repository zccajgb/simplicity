use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::Serialize;
use std::fmt::Display;

pub type ApiResult<T> = Result<T, ApiError>;
pub type ApiJsonResult<T> = Result<Json<T>, ApiError>;

#[derive(Debug, Serialize, Display)]
pub struct ApiError {
    message: String,
    status: u16,
}

impl ApiError {
    pub fn new(message: String, status: u16) -> Self {
        Self { message, status }
    }
}

pub trait ResultExt<T, Error> {
    pub fn map_api_err(&self) -> ApiResult<T>;
}

pub impl<T, E> ResultExt<T, Error> for Result<T, Error> {
    fn map_api_err(&self) -> Result<T, ApiError> {
        self.map_err(|e| ApiError::new(format!({}, e), 500))
    }
}

pub impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status = Status::from_code(self.status).unwrap_or(Status::InternalServerError);
        let json = Json(self);
        let json_str = json::to_string(&json_value.0).unwrap();
        rocket::Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(json_str.len(), std::io::Cursor::new(json_str))
            .ok()
    }
}
