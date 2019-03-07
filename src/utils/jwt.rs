use chrono::{Duration, Utc};
use jwt::{
    decode, encode,
    errors::ErrorKind,
    Header,
    Validation,
};
use uuid::Uuid;
use std::env;

use crate::models::User;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: Uuid,
    pub exp: i64,
}

pub trait CanGenerateJwt {
    fn generate_jwt(&self) -> Result<String>;
}

impl CanGenerateJwt for User {
    fn generate_jwt(&self) -> Result<String> {
        let exp = (Utc::now() + Duration::days(21)).timestamp();
        let claims = Claims {
            id: self.id,
            exp,
        };

        let header = Header::default();
        let secret = &get_secret();
        let token = encode(&header, &claims, secret.as_ref())?;

        Ok(token)
    }
}

fn get_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into())
}
