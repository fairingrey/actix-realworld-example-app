use actix_web::{
    HttpRequest,
    HttpResponse,
    Json,
    Responder,
};
use futures::Future;
use regex::Regex;
use validator::Validate;

use super::AppState;
use crate::models::{NewUser, User};
use crate::db::users::*;
use crate::utils::hasher;
use crate::prelude::*;

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[[:alnum:]]+$").unwrap();
}

// username
//  - length: 1..=20
//  - must be unique
//  - can't be blank
//  - must match /^[a-zA-Z0-9]+$/
// email
//   - must be unique
//   - can't be blank
//   - must match /\S+@\S+\.\S+/
// password
//   - length: 8..=72

// Messages

#[derive(Debug, Validate, Deserialize)]
pub struct SignupUser {
    #[validate(length(min = "1", max = "20"), regex = "RE_USERNAME")]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = "8", max = "72"))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SigninUser {
    pub email: String,
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

// Route handlers

//pub fn sign_up(form: Json<SignupUser>, req: HttpRequest<AppState>) -> impl Future<Item = HttpResponse, Error = Error> {
//    let form = form.into_inner();
//    form.validate();
//
//    let password = hasher().unwrap().hash(&form.password)?;
//
//    let new_user = NewUser {
//        username: form.username.clone(),
//        email: form.email.clone(),
//        password: password,
//        bio: None,
//        image: None,
//    };
//
//    // TODO
//}

