use actix::prelude::*;
use diesel::prelude::*;

use crate::db::DbExecutor;
use crate::prelude::*;
use crate::utils::{
    auth::{Auth, CreateAuth},
    jwt::CanDecodeJwt,
};

// handler implementations ↓

impl Message for CreateAuth {
    type Result = Result<Auth, Error>;
}

impl Handler<CreateAuth> for DbExecutor {
    type Result = Result<Auth, Error>;

    fn handle(&mut self, msg: CreateAuth, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let claims = msg.token.decode_jwt()?.claims;

        let conn = &self.0.get().expect("Connection couldn't be opened");

        match users.find(claims.id).first(conn) {
            Ok(user) => Ok(Auth {
                user,
                token: msg.token,
            }),
            Err(e) => Err(e.into()),
        }
    }
}
