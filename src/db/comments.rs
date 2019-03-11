use actix::prelude::*;
use diesel::prelude::*;

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
        unimplemented!()
    }
}

impl Message for GetComments {
    type Result = Result<CommentListResponse>;
}

impl Handler<GetComments> for DbExecutor {
    type Result = Result<CommentListResponse>;

    fn handle(&mut self, msg: GetComments, _: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}

impl Message for DeleteComment {
    type Result = Result<()>;
}

impl Handler<DeleteComment> for DbExecutor {
    type Result = Result<()>;

    fn handle(&mut self, msg: DeleteComment, _: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}
