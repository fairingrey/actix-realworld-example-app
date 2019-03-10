use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::article_tags;

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(article_id, tag_name)]
pub struct ArticleTag {
    pub article_id: Uuid,
    pub tag_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "article_tags"]
pub struct NewArticleTag {
    pub article_id: Uuid,
    pub tag_name: String,
}
