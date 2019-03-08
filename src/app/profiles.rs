use actix_web::{HttpRequest, HttpResponse, Json, ResponseError};
use futures::{future::result, Future};
use std::convert::From;

use super::AppState;
use crate::models::User;
use crate::prelude::*;

// JSON response objects ↓

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub profile: ProfileResponseInner,
}

#[derive(Debug, Serialize)]
pub struct ProfileResponseInner {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}
