use crate::schema::{article_tags, articles, favorite_articles as fav_articles};
use crate::db::Conn;
use diesel::prelude::*;
use crate::models::*;
use crate::prelude::*;

pub fn insert(conn: &Conn, article: &NewArticle) -> Result<Article> {
    diesel::insert_into(articles::table)
        .values(article)
        .get_result(conn)
        .map_err(|e| e.into())
}
