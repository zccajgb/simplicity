use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::{self, Json};
use serde::Serialize;

pub type ApiResult<T> = Result<T, ApiError>;
pub type ApiJsonResult<T> = Result<Json<T>, ApiError>;

#[derive(Debug, Serialize)]
pub struct ApiError {
    message: String,
    status: u16,
}

impl ApiError {
    pub fn new(error: String, status: u16) -> Self {
        Self {
            message: error,
            status,
        }
    }
}

pub trait ResultExt<T, Error> {
    fn map_api_err(self) -> ApiResult<T>;
}

impl<T, E: std::fmt::Display> ResultExt<T, E> for Result<T, E> {
    fn map_api_err(self) -> Result<T, ApiError> {
        self.inspect_err(|e| error!("Error: {}", e))
            .map_err(|e| ApiError::new(format!("{}", e), 500))
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status = Status::from_code(self.status).unwrap_or(Status::InternalServerError);
        let json = Json(self);
        let json_str = json::to_string(&json.0).unwrap();
        rocket::Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(json_str.len(), std::io::Cursor::new(json_str))
            .ok()
    }
}
