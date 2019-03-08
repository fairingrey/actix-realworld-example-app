use actix_web::{http::header::AUTHORIZATION, HttpRequest};
use futures::{future::result, Future};

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

// message
#[derive(Debug)]
pub struct CreateAuth {
    pub token: String,
}

pub fn authenticate(req: &HttpRequest<AppState>) -> impl Future<Item = Auth, Error = Error> {
    let db = req.state().db.clone();

    result(preprocess_authz_token(req))
        .and_then(move |token| db.send(CreateAuth { token }).from_err())
        .flatten()
}

fn preprocess_authz_token(req: &HttpRequest<AppState>) -> Result<String> {
    let token = match req.headers().get(AUTHORIZATION) {
        Some(token) => token.to_str().unwrap(),
        None => {
            return Err(Error::Unauthorized(
                "No authorization header provided".to_string(),
            ))
        }
    };

    if !token.starts_with(TOKEN_PREFIX) {
        return Err(Error::Unauthorized(
            "Invalid authorization field".to_string(),
        ));
    }

    let token = token.replacen(TOKEN_PREFIX, "", 1);

    Ok(token)
}
