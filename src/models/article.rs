use chrono::NaiveDateTime;

use crate::schema::{articles, favorite_articles};

#[derive(Debug, Queryable)]
pub struct Article {
    pub id: i32,
    pub author_id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "articles"]
pub struct NewArticle {
    pub author_id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
}

#[cfg(test)]
impl Default for NewArticle {
    fn default() -> Self {
        NewArticle {
            author_id: 0,
            slug: String::new(),
            title: String::new(),
            description: String::new(),
            body: String::new(),
        }
    }
}

// XXX: One can create an ArticleChange with title but without slug.
// It should be avoided.
#[derive(Debug, AsChangeset)]
#[table_name = "articles"]
pub struct ArticleChange {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Insertable)]
#[table_name = "favorite_articles"]
pub struct NewFavoriteArticle {
    pub user_id: i32,
    pub article_id: i32,
}
