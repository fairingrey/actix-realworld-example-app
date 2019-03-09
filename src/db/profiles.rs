use actix::prelude::*;
use diesel::prelude::*;

use super::DbExecutor;
use crate::app::profiles::{GetProfile, ProfileResponse, ProfileResponseInner};
use crate::models::{Follower, User};
use crate::prelude::*;

// handler implementations ↓

impl Message for GetProfile {
    type Result = Result<ProfileResponse>;
}

impl Handler<GetProfile> for DbExecutor {
    type Result = Result<ProfileResponse>;

    fn handle(&mut self, msg: GetProfile, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get().expect("Connection couldn't be opened");

        let user: User = {
            use crate::schema::users::dsl::*;
            users.filter(username.eq(msg.username)).first(conn)?
        };

        use crate::schema::followers::dsl::*;

        let following = match msg.auth {
            Some(auth) => followers
                .filter(user_id.eq(user.id))
                .filter(follower_id.eq(auth.user.id))
                .first::<Follower>(conn)
                .optional()?
                .is_some(),
            None => false,
        };

        Ok(ProfileResponse {
            profile: ProfileResponseInner {
                username: user.username,
                bio: user.bio,
                image: user.image,
                following,
            },
        })
    }
}
