use actix_web::{web::Data, web::Json, HttpRequest, HttpResponse};
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
            min = 1,
            max = 20,
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
        min = 8,
        max = 72,
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginUser {
    #[validate(email(message = "fails validation - is not a valid email address"))]
    pub email: String,
    #[validate(length(
        min = 8,
        max = 72,
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateUser {
    #[validate(
        length(
            min = 1,
            max = 20,
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
        min = 8,
        max = 72,
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: Option<String>,
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
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

pub async fn register(
    (form, state): (Json<In<RegisterUser>>, Data<AppState>),
) -> Result<HttpResponse, Error> {
    let register_user = form.into_inner().user;
    register_user.validate()?;

    let res = state.db.send(register_user).await??;
    Ok(HttpResponse::Ok().json(res))
}

pub async fn login(
    (form, state): (Json<In<LoginUser>>, Data<AppState>),
) -> Result<HttpResponse, Error> {
    let login_user = form.into_inner().user;
    login_user.validate()?;

    let res = state.db.send(login_user).await??;
    Ok(HttpResponse::Ok().json(res))
}

pub async fn get_current(state: Data<AppState>, req: HttpRequest) -> Result<HttpResponse, Error> {
    authenticate(&state, &req)
        .await
        .and_then(|auth| Ok(HttpResponse::Ok().json(UserResponse::create_with_auth(auth))))
}

pub async fn update(
    state: Data<AppState>,
    (form, req): (Json<In<UpdateUser>>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let update_user = form.into_inner().user;
    update_user.validate()?;

    let auth = authenticate(&state, &req).await?;
    let res = state
        .db
        .send(UpdateUserOuter { auth, update_user })
        .await??;
    Ok(HttpResponse::Ok().json(res))
}
