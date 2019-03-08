use chrono::{Duration, Utc};
use jwt::{decode, encode, Header, TokenData, Validation};
use std::env;
use uuid::Uuid;

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
        let claims = Claims { id: self.id, exp };

        let header = Header::default();
        let secret = &get_secret();
        let token = encode(&header, &claims, secret.as_ref())?;

        Ok(token)
    }
}

pub trait CanDecodeJwt {
    fn decode_jwt(&self) -> Result<TokenData<Claims>>;
}

impl CanDecodeJwt for String {
    fn decode_jwt(&self) -> Result<TokenData<Claims>> {
        match decode::<Claims>(&self, get_secret().as_ref(), &Validation::default()) {
            Ok(res) => Ok(res),
            Err(e) => Err(e.into()),
        }
    }
}

fn get_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into())
}
