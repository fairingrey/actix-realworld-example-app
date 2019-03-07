use actix_web::{http::header::AUTHORIZATION, FromRequest, HttpRequest};

use crate::models::User;
use crate::utils::jwt::{Claims, CanDecodeJwt};
use crate::prelude::*;

const TOKEN_PREFIX: &str = "Token ";

#[derive(Debug)]
pub struct Auth {
    pub user: User,
}

pub trait CanAuthenticate {
    fn authenticate<S>(&self, req: &HttpRequest<S>) -> Result<Auth>;
}

impl<T: CanDecodeJwt> CanAuthenticate for T {
    fn authenticate<AppState>(&self, req: &HttpRequest<AppState>) -> Result<Auth> {

        // Check for existing token on authorization header
        let token = match req.headers().get(AUTHORIZATION) {
            Some(token) => token.to_str().unwrap(),
            None => return Err(Error::Unauthorized("An authorization header was present but nothing was provided".to_string())),
        };

        if !token.starts_with(TOKEN_PREFIX) {
            return Err(Error::Unauthorized("Invalid Authorization method".to_string()));
        }
        let token = token.replacen(TOKEN_PREFIX, "", 1);

        unimplemented!()
    }
}
