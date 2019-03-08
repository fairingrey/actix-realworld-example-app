use actix::prelude::*;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use crate::db::DbExecutor;
use crate::models::{FindUserById, NewUser, User, UserChange};
use crate::prelude::*;

impl Message for NewUser {
    type Result = Result<User, Error>;
}

impl Handler<NewUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, new_user: NewUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let conn = &self.0.get().expect("Connection couldn't be opened");

        let user: Result<User, DieselError> =
            diesel::insert_into(users).values(new_user).get_result(conn);

        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(e.into()),
        }
    }
}

impl Message for FindUserById {
    type Result = Result<User, Error>;
}

impl Handler<FindUserById> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, find_user: FindUserById, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let conn = &self.0.get().expect("Connection couldn't be opened");

        let user: Result<User, DieselError> = users.find(id).first(conn);

        match user {
            Ok(user) => Ok(user),
            Err(e) => Err(e.into()),
        }
    }
}
