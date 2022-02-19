use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Internal server error")]
    Internal,
    #[error("Home is not found")]
    HomeNotFound,
    #[error("Home is already exists")]
    HomeAlreadyExists,
    #[error("Room is not found")]
    RoomNotFound,
    #[error("Room is alredy exists")]
    RoomAlreadyExists,
    #[error("Device is not found")]
    DeviceNotFound,
    #[error("Device is already exists")]
    DeviceAlreadyExists,
    #[error("Device type is not compatible")]
    DeviceNotCompatible,
}

impl Error {
    pub fn name(&self) -> String {
        match self {
            Error::Internal => "InternalError",
            Error::HomeNotFound => "HomeNotFound",
            Error::HomeAlreadyExists => "HomeAlreadyExists",
            Error::RoomNotFound => "RoomNotFound",
            Error::RoomAlreadyExists => "RoomAlreadyExists",
            Error::DeviceNotFound => "DeviceNotFound",
            Error::DeviceAlreadyExists => "DeviceAlreadyExists",
            Error::DeviceNotCompatible => "DeviceNotCompatible",
        }
        .to_string()
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Error::HomeNotFound => StatusCode::NOT_FOUND,
            Error::HomeAlreadyExists => StatusCode::FORBIDDEN,
            Error::RoomNotFound => StatusCode::NOT_FOUND,
            Error::RoomAlreadyExists => StatusCode::FORBIDDEN,
            Error::DeviceNotFound => StatusCode::NOT_FOUND,
            Error::DeviceAlreadyExists => StatusCode::FORBIDDEN,
            Error::DeviceNotCompatible => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

pub type WebResult<T> = Result<T, Error>;
