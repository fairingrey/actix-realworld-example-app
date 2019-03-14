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

    fn handle(&mut self, _msg: GetTags, _: &mut Self::Context) -> Self::Result {
        use crate::schema::article_tags::dsl::*;

        let conn = &self.0.get()?;

        let tags = article_tags
            .distinct_on(tag_name)
            .load::<ArticleTag>(conn)?;

        let tag_list = tags
            .iter()
            .map(|tag| tag.tag_name.to_owned())
            .collect::<Vec<String>>();

        Ok(TagsResponse { tags: tag_list })
    }
}
