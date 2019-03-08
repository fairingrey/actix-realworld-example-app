use actix_web::{http::header::AUTHORIZATION, HttpRequest};
use futures::future::Future;
use uuid::Uuid;

use crate::app::AppState;
use crate::models::User;
use crate::prelude::*;
use crate::utils::jwt::CanDecodeJwt;

const TOKEN_PREFIX: &str = "Token ";

// expand this as needed
#[derive(Debug)]
pub struct Auth {
    pub user: User,
}

// message
#[derive(Debug)]
pub struct FindUserById {
    pub id: Uuid,
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

        let claims = token.decode_jwt()?.claims;

        let user = self
            .state()
            .db
            .send(FindUserById { id: claims.id })
            .from_err::<Error>()
            .wait()??;

        Ok(Auth { user })
    }
}
