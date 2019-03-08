use actix::prelude::*;
use diesel::prelude::*;
use libreauth::pass::HashBuilder;

use crate::app::users::{LoginUser, RegisterUser, UpdateUserOuter};
use crate::db::DbExecutor;
use crate::models::{NewUser, User, UserChange};
use crate::prelude::*;
use crate::utils::{hasher, PWD_SCHEME_VERSION};

impl Message for RegisterUser {
    type Result = Result<User, Error>;
}

impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: RegisterUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let new_user = NewUser {
            username: msg.username.clone(),
            email: msg.email.clone(),
            password: hasher().hash(&msg.password).unwrap(),
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

impl Message for LoginUser {
    type Result = Result<User, Error>;
}

impl Handler<LoginUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: LoginUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let provided_password_raw = &msg.password;

        let conn = &self.0.get().expect("Connection couldn't be opened");

        let stored_user: User = users.filter(email.eq(msg.email)).first(conn)?;
        let checker = HashBuilder::from_phc(&stored_user.password)?;

        if checker.is_valid(provided_password_raw) {
            if checker.needs_update(PWD_SCHEME_VERSION) {
                let new_password = hasher().hash(provided_password_raw)?;
                return match diesel::update(users.find(stored_user.id))
                    .set(password.eq(new_password))
                    .get_result(conn)
                {
                    Ok(user) => Ok(user),
                    Err(e) => Err(e.into()),
                };
            }
            Ok(stored_user)
        } else {
            Err(Error::Unauthorized("Wrong password".to_string()))
        }
    }
}

impl Message for UpdateUserOuter {
    type Result = Result<User, Error>;
}

impl Handler<UpdateUserOuter> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: UpdateUserOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let auth = msg.auth;
        let update_user = msg.update_user;

        let conn = &self.0.get().expect("Connection couldn't be opened");

        let updated_password = match update_user.password {
            Some(updated_password) => Some(hasher().hash(&updated_password)?),
            None => None,
        };

        let updated_user = UserChange {
            username: update_user.username,
            email: update_user.email,
            password: updated_password,
            bio: update_user.bio,
            image: update_user.image,
        };

        match diesel::update(users.find(auth.user.id))
            .set(&updated_user)
            .get_result(conn)
        {
            Ok(user) => Ok(user),
            Err(e) => Err(e.into()),
        }
    }
}
