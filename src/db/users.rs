use actix::prelude::*;
use diesel::prelude::*;
use libreauth::pass::HashBuilder;

use crate::app::users::{SigninUser, SignupUser};
use crate::db::DbExecutor;
use crate::models::{NewUser, User, UserChange};
use crate::prelude::*;
use crate::utils::{auth::FindUserById, hasher, PWD_SCHEME_VERSION};

impl Message for SignupUser {
    type Result = Result<User, Error>;
}

impl Handler<SignupUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, signup_user: SignupUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let new_user = NewUser {
            username: signup_user.username.clone(),
            email: signup_user.email.clone(),
            password: hasher().hash(&signup_user.password).unwrap(),
            bio: None,
            image: None,
        };

        let conn = &self.0.get().expect("Connection couldn't be opened");

        match diesel::insert_into(users).values(new_user).get_result(conn) {
            Ok(user) => Ok(user),
            Err(e) => Err(e.into()),
        }
    }
}

impl Message for SigninUser {
    type Result = Result<User, Error>;
}

impl Handler<SigninUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, signin_user: SigninUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let provided_password_raw = &signin_user.password;

        let conn = &self.0.get().expect("Connection couldn't be opened");

        let mut stored_user: User = users.filter(email.eq(signin_user.email)).first(conn)?;
        let checker = HashBuilder::from_phc(&stored_user.password).unwrap();

        if checker.is_valid(provided_password_raw) {
            if checker.needs_update(PWD_SCHEME_VERSION) {
                let new_password = hasher().hash(provided_password_raw)?;
                stored_user = diesel::update(users.find(stored_user.id))
                    .set(password.eq(new_password))
                    .get_result(conn)?
            }
            Ok(stored_user)
        } else {
            Err(Error::Unauthorized("Wrong password".to_string()))
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

        match users.find(find_user.id).first(conn) {
            Ok(user) => Ok(user),
            Err(e) => Err(e.into()),
        }
    }
}
