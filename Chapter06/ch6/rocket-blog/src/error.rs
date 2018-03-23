use std::error::Error;
use std::convert::From;
use std::fmt;
use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::response::{Response, Responder};
use rocket::Request;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InternalServerError,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::NotFound => f.write_str("NotFound"),
            ApiError::InternalServerError => f.write_str("InternalServerError"),
        }
    }
}

impl From<DieselError> for ApiError {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => ApiError::NotFound,
            _ => ApiError::InternalServerError,
        }
    }
}

impl Error for ApiError {
    fn description(&self) -> &str {
        match *self {
            ApiError::NotFound => "Record not found",
            ApiError::InternalServerError => "Internal server error",
        }
    }
}


impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        match self {
            ApiError::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}
