use actix::prelude::*;
use diesel::prelude::*;

use super::DbExecutor;
use crate::app::tags::{GetTags, TagsResponse};
use crate::models::ArticleTag;
use crate::prelude::*;

impl Message for GetTags {
    type Result = Result<TagsResponse>;
}

impl Handler<GetTags> for DbExecutor {
    type Result = Result<TagsResponse>;

    fn handle(&mut self, msg: GetTags, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        // TODO

        unimplemented!()
    }
}
