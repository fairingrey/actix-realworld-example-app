use actix_web::{
    error::ResponseError,
    HttpResponse,
};
use std::convert::From;
use diesel::result::{
    DatabaseErrorKind,
    Error as DieselError,
};
use libreauth::pass::ErrorCode as PassErrorCode;

