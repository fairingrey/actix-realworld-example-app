use actix::prelude::*;
use diesel::prelude::*;

use crate::db::DbExecutor;
use crate::prelude::*;
use crate::utils::{
    auth::{Auth, GenerateAuth},
    jwt::CanDecodeJwt,
};

// message handler implementations ↓

impl Message for GenerateAuth {
    type Result = Result<Auth>;
}

impl Handler<GenerateAuth> for DbExecutor {
    type Result = Result<Auth>;

    fn handle(&mut self, msg: GenerateAuth, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let claims = msg.token.decode_jwt()?.claims;

        let conn = &self.0.get()?;

        match users.find(claims.id).first(conn) {
            Ok(user) => Ok(Auth {
                user,
                token: msg.token,
            }),
            Err(e) => Err(e.into()),
        }
    }
}
