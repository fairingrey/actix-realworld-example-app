use actix_web::{
    error::ResponseError,
    HttpResponse,
};
use diesel::result::{
    DatabaseErrorKind,
    Error as DieselError,
};
use libreauth::pass::ErrorCode as PassErrorCode;
use validator::{
    ValidationError,
    ValidationErrors
};
use std::convert::From;

// more error types can be found at below link but we should only need these for now
// https://actix.rs/actix-web/actix_web/struct.HttpResponse.html
#[derive(Fail, Debug)]
pub enum ConduitError {
    #[fail(display = "Internal Server Error")]
    InternalServerError,

    #[fail(display = "Bad Request: {}", _0)]
    BadRequest(String),
}

// the ResponseError trait lets us convert errors to http responses with appropriate data
// https://actix.rs/docs/errors/
impl ResponseError for ConduitError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ConduitError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            },
            ConduitError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}

impl From<DieselError> for ConduitError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ConduitError::BadRequest(message);
                }
                ConduitError::InternalServerError
            }
            _ => ConduitError::InternalServerError
        }
    }
}

impl From<PassErrorCode> for ConduitError {
    fn from(_: PassErrorCode) -> Self {
        ConduitError::BadRequest("Invalid password".to_string())
    }
}

impl From<ValidationError> for ConduitError {
    fn from(error: ValidationError) -> Self {
        ConduitError::BadRequest(format!("Validation failed on some constraint.\n{:?}", error))
    }
}

impl From<ValidationErrors> for ConduitError {
    fn from(errors: ValidationErrors) -> Self {
        ConduitError::BadRequest(format!("Validation failed on some fields.\n{:?}", errors))
    }
}
