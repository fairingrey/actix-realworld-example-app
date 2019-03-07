use actix_web::{
    actix::MailboxError,
    error::{
        self,
        ResponseError,
    },
    http::StatusCode,
    HttpResponse,
};
use diesel::{
    result::{
        DatabaseErrorKind,
        Error as DieselError,
    },
    r2d2::PoolError,
};
use jwt::errors::Error as JwtError;
use libreauth::pass::ErrorCode as PassErrorCode;
use validator::{
    ValidationError,
    ValidationErrors,
};
use std::convert::From;

// more error types can be found at below link but we should only need these for now
// https://actix.rs/actix-web/actix_web/struct.HttpResponse.html
#[derive(Fail, Debug)]
pub enum Error {

    // 400
    #[fail(display = "Bad Request: {}", _0)]
    BadRequest(String),

    // 401
    #[fail(display = "Unauthorized: {}", _0)]
    Unauthorized(String),

    // 403
    #[fail(display = "Forbidden: {}", _0)]
    Forbidden(String),

    // 422
    #[fail(display = "Unprocessable Entity: {}", _0)]
    UnprocessableEntity(String),

    // 500
    #[fail(display = "Internal Server Error")]
    InternalServerError,

}

// the ResponseError trait lets us convert errors to http responses with appropriate data
// https://actix.rs/docs/errors/
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            Error::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
            Error::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
            Error::UnprocessableEntity(ref message) => HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(message),
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            },
        }
    }
}

impl From<MailboxError> for Error {
    fn from(error: MailboxError) -> Self {
        Error::InternalServerError
    }
}

impl From<JwtError> for Error {
    fn from(error: JwtError) -> Self {
        Error::BadRequest("JSON web token provided is invalid".to_string())
    }
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::UnprocessableEntity(message);
                }
                Error::InternalServerError
            }
            _ => Error::InternalServerError
        }
    }
}

impl From<PoolError> for Error {
    fn from(error: PoolError) -> Self {
        Error::InternalServerError
    }
}

impl From<PassErrorCode> for Error {
    fn from(_error: PassErrorCode) -> Self {
        Error::BadRequest("Invalid password provided.".to_string())
    }
}

impl From<ValidationError> for Error {
    fn from(_error: ValidationError) -> Self {
        Error::BadRequest("Validation failed on some constraint.".to_string())
    }
}

impl From<ValidationErrors> for Error {
    fn from(_errors: ValidationErrors) -> Self {
        Error::BadRequest("Validation failed on some fields.".to_string())
    }
}
