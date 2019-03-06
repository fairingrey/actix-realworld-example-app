use actix::prelude::*;
use diesel::prelude::*;

use crate::db::DbExecutor;
use crate::models::{NewUser, User, UserChange};
use crate::prelude::*;
use crate::schema::users;

// Note that the password should be passed in already hashed by crate::utils::hasher.

impl Message for NewUser {
    type Result = Result<User, Error>;
}

impl Handler<NewUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: NewUser, _: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}
