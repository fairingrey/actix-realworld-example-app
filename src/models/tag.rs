use crate::schema::article_tags;

#[derive(Debug, Insertable)]
#[table_name = "article_tags"]
pub struct NewArticleTag {
    pub article_id: i32,
    pub tag_name: String,
}
