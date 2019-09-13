use actix_web::{HttpRequest, HttpResponse, web::Json, ResponseError, web::Data};
use futures::{future::result, Future};
use regex::Regex;
use std::convert::From;
use validator::Validate;

use super::AppState;
use crate::models::User;
use crate::prelude::*;
use crate::utils::{
    auth::{authenticate, Auth},
    jwt::CanGenerateJwt,
};

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[_0-9a-zA-Z]+$").unwrap();
}

#[derive(Debug, Deserialize)]
pub struct In<U> {
    user: U,
}

// Client Messages ↓

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUser {
    #[validate(
        length(
            min = "1",
            max = "20",
            message = "fails validation - must be 1-20 characters long"
        ),
        regex(
            path = "RE_USERNAME",
            message = "fails validation - is not only alphanumeric/underscore characters"
        )
    )]
    pub username: String,
    #[validate(email(message = "fails validation - is not a valid email address"))]
    pub email: String,
    #[validate(length(
        min = "8",
        max = "72",
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginUser {
    #[validate(email(message = "fails validation - is not a valid email address"))]
    pub email: String,
    #[validate(length(
        min = "8",
        max = "72",
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUser {
    #[validate(
        length(
            min = "1",
            max = "20",
            message = "fails validation - must be 1-20 characters long"
        ),
        regex(
            path = "RE_USERNAME",
            message = "fails validation - is not only alphanumeric/underscore characters"
        )
    )]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(
        min = "8",
        max = "72",
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: Option<String>,
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub bio: Option<String>,
    #[validate(url(message = "is not a URL"))]
    pub image: Option<String>,
}

#[derive(Debug)]
pub struct UpdateUserOuter {
    pub auth: Auth,
    pub update_user: UpdateUser,
}

// JSON response objects ↓

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
    fn create_with_auth(auth: Auth) -> Self {
        UserResponse {
            user: UserResponseInner {
                token: auth.token,
                email: auth.user.email,
                username: auth.user.username,
                bio: auth.user.bio,
                image: auth.user.image,
            },
        }
    }
}

// Route handlers ↓

pub fn register(
    (form, state): (Json<In<RegisterUser>>, Data<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let register_user = form.into_inner().user;

    result(register_user.validate())
        .from_err()
        .and_then(move |_| state.db.send(register_user).from_err())
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn login(
    (form, state): (Json<In<LoginUser>>, Data<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let login_user = form.into_inner().user;

    result(login_user.validate())
        .from_err()
        .and_then(move |_| state.db.send(login_user).from_err())
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn get_current(state: Data<AppState>, req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    authenticate(&state, &req)
        .and_then(|auth| Ok(HttpResponse::Ok().json(UserResponse::create_with_auth(auth))))
}

pub fn update(
    state: Data<AppState>,
    (form, req): (Json<In<UpdateUser>>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let update_user = form.into_inner().user;

    let db = state.db.clone();

    result(update_user.validate())
        .from_err()
        .and_then(move |_| authenticate(&state, &req))
        .and_then(move |auth| db.send(UpdateUserOuter { auth, update_user }).from_err())
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}
