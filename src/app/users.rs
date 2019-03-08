use actix_web::{HttpRequest, HttpResponse, Json, ResponseError};
use futures::{future::result, Future};
use libreauth::pass::ErrorCode as PassErrorCode;
use regex::Regex;
use std::convert::From;
use validator::Validate;

use super::AppState;
use crate::models::User;
use crate::prelude::*;
use crate::utils::{hasher, jwt::CanGenerateJwt};

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[[:alnum:]]+$").unwrap();
}

#[derive(Debug, Deserialize)]
pub struct In<U> {
    user: U,
}

// Client Messages

#[derive(Debug, Validate, Deserialize)]
pub struct SignupUser {
    #[validate(length(min = "1", max = "20"), regex = "RE_USERNAME")]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "8", max = "72"))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct SigninUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "8", max = "72"))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UserChange {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = "1", max = "20"), regex = "RE_USERNAME")]
    pub username: Option<String>,
    pub bio: Option<String>,
    #[validate(url)]
    pub image: Option<String>,
    #[validate(length(min = "8", max = "72"))]
    pub password: Option<String>,
}

// JSON response objects

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user: UserResponseInner,
}

#[derive(Debug, Serialize)]
pub struct UserResponseInner {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

// The spec requires the response be in this order
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            user: UserResponseInner {
                token: user.generate_jwt().unwrap(),
                email: user.email,
                username: user.username,
                bio: user.bio,
                image: user.image,
            },
        }
    }
}

impl UserResponse {
    fn create_with_token(token: String, user: User) -> Self {
        UserResponse {
            user: UserResponseInner {
                token,
                email: user.email,
                username: user.username,
                bio: user.bio,
                image: user.image,
            },
        }
    }
}

// Route handlers

pub fn sign_up(
    (form, req): (Json<In<SignupUser>>, HttpRequest<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let signup_user = form.into_inner().user;

    let db = req.state().db.clone();

    result(signup_user.validate())
        .from_err()
        .and_then(move |_| db.send(signup_user).from_err())
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(UserResponse::from(user))),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn sign_in(
    (form, req): (Json<In<SigninUser>>, HttpRequest<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let signin_user = form.into_inner().user;

    req.state()
        .db
        .send(signin_user)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(UserResponse::from(user))),
            Err(e) => Ok(e.error_response()),
        })
}
