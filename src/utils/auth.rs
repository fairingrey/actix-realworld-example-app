use actix_web::{http::header::AUTHORIZATION, HttpRequest};
use futures::{future::result, Future};
//use serde_json::Value as JsonValue;

use crate::app::AppState;
use crate::models::User;
use crate::prelude::*;

const TOKEN_PREFIX: &str = "Token ";

// expand this as needed
#[derive(Debug)]
pub struct Auth {
    pub user: User,
    pub token: String,
}

// create auth message
#[derive(Debug)]
pub struct GenerateAuth {
    pub token: String,
}

pub fn authenticate(req: &HttpRequest<AppState>) -> impl Future<Item = Auth, Error = Error> {
    let db = req.state().db.clone();

    result(preprocess_authz_token(req))
        .and_then(move |token| db.send(GenerateAuth { token }).from_err())
        .flatten()
}

fn preprocess_authz_token(req: &HttpRequest<AppState>) -> Result<String> {
    let token = match req.headers().get(AUTHORIZATION) {
        Some(token) => token.to_str().unwrap(),
        None => return Err(Error::Unauthorized(json!("No authorization was provided"))),
    };

    if !token.starts_with(TOKEN_PREFIX) {
        return Err(Error::Unauthorized(json!("Invalid authorization method")));
    }

    let token = token.replacen(TOKEN_PREFIX, "", 1);

    Ok(token)
}
