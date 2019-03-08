use actix_web::{http::header::AUTHORIZATION, HttpRequest};
use futures::future::Future;

use crate::app::AppState;
use crate::models::{User, FindUserById};
use crate::prelude::*;
use crate::utils::jwt::CanDecodeJwt;

const TOKEN_PREFIX: &str = "Token ";

#[derive(Debug)]
pub struct Auth {
    pub user: User,
}

pub trait CanAuthenticate {
    fn authenticate(&self) -> Result<Auth>;
}

impl CanAuthenticate for HttpRequest<AppState> {
    fn authenticate(&self) -> Result<Auth> {
        // Check for existing token on authorization header
        let token = match self.headers().get(AUTHORIZATION) {
            Some(token) => token.to_str().unwrap(),
            None => {
                return Err(Error::Unauthorized(
                    "An authorization header was present but nothing was provided".to_string(),
                ))
            }
        };

        if !token.starts_with(TOKEN_PREFIX) {
            return Err(Error::Unauthorized(
                "Invalid Authorization method".to_string(),
            ));
        }
        let token = token.replacen(TOKEN_PREFIX, "", 1);

        let claims = token.decode_jwt()?.claims;

        let user = self.state().db.send(FindUserById {
            id: claims.id,
        }).from_err::<Error>().wait()??;

        Ok(Auth {
            user,
        })
    }
}
