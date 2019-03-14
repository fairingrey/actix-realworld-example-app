use actix::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;

use super::{DbExecutor, PooledConn};
use crate::app::articles::comments::{
    AddCommentOuter, CommentListResponse, CommentResponse, CommentResponseInner, DeleteComment,
    GetComments,
};
use crate::app::profiles::ProfileResponseInner;
use crate::models::{Comment, Follower, NewComment, User};
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

        get_comment_response(comment.id, Some(user_id), conn)
    }
}

impl Message for GetComments {
    type Result = Result<CommentListResponse>;
}

impl Handler<GetComments> for DbExecutor {
    type Result = Result<CommentListResponse>;

    fn handle(&mut self, msg: GetComments, _: &mut Self::Context) -> Self::Result {
        use crate::schema::{articles, comments};

        let conn = &self.0.get()?;

        let article_id = articles::table
            .filter(articles::slug.eq(msg.slug))
            .select(articles::id)
            .get_result::<Uuid>(conn)?;

        let comments = comments::table
            .filter(comments::article_id.eq(article_id))
            .load::<Comment>(conn)?;

        match msg.auth {
            Some(auth) => get_comment_list_response(comments, Some(auth.user.id), conn),
            None => get_comment_list_response(comments, None, conn),
        }
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

fn get_comment_response(
    comment_id: i32,
    user_id: Option<Uuid>,
    conn: &PooledConn,
) -> Result<CommentResponse> {
    use crate::schema::{comments, followers, users};

    let (comment, commenter) = comments::table
        .inner_join(users::table)
        .filter(comments::id.eq(comment_id))
        .get_result::<(Comment, User)>(conn)?;

    let following = match user_id {
        Some(user_id) => followers::table
            .filter(followers::user_id.eq(commenter.id))
            .filter(followers::follower_id.eq(user_id))
            .first::<Follower>(conn)
            .optional()?
            .is_some(),
        None => false,
    };

    Ok(CommentResponse {
        comment: CommentResponseInner {
            id: comment.id,
            created_at: CustomDateTime(comment.created_at),
            updated_at: CustomDateTime(comment.updated_at),
            body: comment.body,
            author: ProfileResponseInner {
                username: commenter.username,
                bio: commenter.bio,
                image: commenter.image,
                following,
            },
        },
    })
}

fn get_comment_list_response(
    comments: Vec<Comment>,
    user_id: Option<Uuid>,
    conn: &PooledConn,
) -> Result<CommentListResponse> {
    let comment_list = comments
        .iter()
        .map(
            |comment| match get_comment_response(comment.id.to_owned(), user_id, conn) {
                Ok(response) => Ok(response.comment),
                Err(e) => Err(e),
            },
        )
        .collect::<Result<Vec<CommentResponseInner>>>()?;

    Ok(CommentListResponse {
        comments: comment_list,
    })
}
