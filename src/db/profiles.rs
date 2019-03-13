use actix::prelude::*;
use diesel::prelude::*;

use super::DbExecutor;
use crate::app::profiles::{
    FollowProfile, GetProfile, ProfileResponse, ProfileResponseInner, UnfollowProfile,
};
use crate::models::{Follower, NewFollower, User};
use crate::prelude::*;

// message handler implementations ↓

impl Message for GetProfile {
    type Result = Result<ProfileResponse>;
}

impl Handler<GetProfile> for DbExecutor {
    type Result = Result<ProfileResponse>;

    fn handle(&mut self, msg: GetProfile, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

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

impl Message for FollowProfile {
    type Result = Result<ProfileResponse>;
}

impl Handler<FollowProfile> for DbExecutor {
    type Result = Result<ProfileResponse>;

    fn handle(&mut self, msg: FollowProfile, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let user_a: User = {
            use crate::schema::users::dsl::*;
            users.filter(username.eq(msg.username)).first(conn)?
        };
        let user_b: User = msg.auth.user;

        if user_a.id == user_b.id {
            return Err(Error::UnprocessableEntity(
                json!({"error": "You cannot follow yourself"}),
            ));
        }

        use crate::schema::followers::dsl::*;

        diesel::insert_into(followers)
            .values(&NewFollower {
                user_id: user_a.id,
                follower_id: user_b.id,
            })
            .execute(conn)?;

        Ok(ProfileResponse {
            profile: ProfileResponseInner {
                username: user_a.username,
                bio: user_a.bio,
                image: user_a.image,
                following: true,
            },
        })
    }
}

impl Message for UnfollowProfile {
    type Result = Result<ProfileResponse>;
}

impl Handler<UnfollowProfile> for DbExecutor {
    type Result = Result<ProfileResponse>;

    fn handle(&mut self, msg: UnfollowProfile, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        let user_a: User = {
            use crate::schema::users::dsl::*;
            users.filter(username.eq(msg.username)).first(conn)?
        };
        let user_b: User = msg.auth.user;

        use crate::schema::followers::dsl::*;

        diesel::delete(
            followers
                .filter(user_id.eq(user_a.id))
                .filter(follower_id.eq(user_b.id)),
        )
        .execute(conn)?;

        Ok(ProfileResponse {
            profile: ProfileResponseInner {
                username: user_a.username,
                bio: user_a.bio,
                image: user_a.image,
                following: false,
            },
        })
    }
}
