use actix::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;

use super::DbExecutor;
use crate::app::articles::comments::{
    AddCommentOuter, CommentListResponse, CommentResponse, CommentResponseInner, DeleteComment,
    GetComments,
};
use crate::app::profiles::ProfileResponseInner;
use crate::models::{Comment, NewComment};
use crate::prelude::*;
use crate::utils::CustomDateTime;

// message handler implementations ↓

impl Message for AddCommentOuter {
    type Result = Result<CommentResponse>;
}

impl Handler<AddCommentOuter> for DbExecutor {
    type Result = Result<CommentResponse>;

    fn handle(&mut self, msg: AddCommentOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::{articles, comments};

        let conn = &self.0.get()?;

        let article_id = articles::table
            .filter(articles::slug.eq(msg.slug))
            .select(articles::id)
            .get_result::<Uuid>(conn)?;

        let user_id = msg.auth.user.id;

        let new_comment = NewComment {
            article_id,
            user_id,
            body: msg.comment.body,
        };

        let comment = diesel::insert_into(comments::table)
            .values(new_comment)
            .get_result::<Comment>(conn)?;

        Ok(CommentResponse {
            comment: CommentResponseInner {
                id: comment.id,
                created_at: CustomDateTime(comment.created_at),
                updated_at: CustomDateTime(comment.updated_at),
                body: comment.body,
                author: ProfileResponseInner {
                    username: msg.auth.user.username,
                    bio: msg.auth.user.bio,
                    image: msg.auth.user.image,
                    following: false,
                },
            },
        })
    }
}

impl Message for GetComments {
    type Result = Result<CommentListResponse>;
}

impl Handler<GetComments> for DbExecutor {
    type Result = Result<CommentListResponse>;

    fn handle(&mut self, msg: GetComments, _: &mut Self::Context) -> Self::Result {
        use crate::schema::{articles, comments, users};

        let conn = &self.0.get()?;

        let article_id = articles::table
            .filter(articles::slug.eq(msg.slug))
            .select(articles::id)
            .get_result::<Uuid>(conn)?;

        let comments = comments::table
            .filter(comments::article_id.eq(article_id))
            .load::<Comment>(conn)?;

        // TODO

        unimplemented!()
    }
}

impl Message for DeleteComment {
    type Result = Result<()>;
}

impl Handler<DeleteComment> for DbExecutor {
    type Result = Result<()>;

    fn handle(&mut self, msg: DeleteComment, _: &mut Self::Context) -> Self::Result {
        use crate::schema::comments::dsl::*;

        let conn = &self.0.get()?;

        let comment = comments
            .filter(id.eq(msg.comment_id))
            .get_result::<Comment>(conn)?;

        if msg.auth.user.id != comment.user_id {
            return Err(Error::Forbidden(json!({
                "error": "user did not make this comment",
            })));
        }

        diesel::delete(comments.filter(id.eq(comment.id))).execute(conn)?;

        Ok(())
    }
}
