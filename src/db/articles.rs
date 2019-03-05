use crate::schema::{article_tags, articles, favorite_articles as fav_articles};
use crate::db::{Conn, may_update};
use diesel::prelude::*;
use crate::models::*;
use crate::prelude::*;

pub fn insert(conn: &Conn, article: &NewArticle) -> Result<Article> {
    diesel::insert_into(articles::table)
        .values(article)
        .get_result(conn)
        .map_err(|e| e.into())
}

pub fn update(conn: &Conn, id: i32, change: &ArticleChange) -> Result<()> {
    may_update(
        diesel::update(articles::table.filter(articles::id.eq(id)))
            .set(change)
            .execute(conn),
    )?;
    Ok(())
}

pub fn delete(conn: &Conn, id: i32) -> Result<()> {
    diesel::delete(articles::table.filter(articles::id.eq(id))).execute(conn)?;
    Ok(())
}

pub fn favorite(conn: &Conn, id: i32, user_id: i32) -> Result<()> {
    let new_favorite = NewFavoriteArticle {
        user_id: user_id,
        article_id: id,
    };
    diesel::insert_into(fav_articles::table)
        .values(&new_favorite)
        .on_conflict((fav_articles::user_id, fav_articles::article_id))
        .do_nothing()
        .execute(conn)?;

    Ok(())
}

pub fn unfavorite(conn: &Conn, id: i32, user_id: i32) -> Result<()> {
    diesel::delete(
        fav_articles::table
            .filter(fav_articles::user_id.eq(user_id))
            .filter(fav_articles::article_id.eq(id)),
    ).execute(conn)?;

    Ok(())
}

pub fn add_tags<I>(conn: &Conn, id: i32, tags: I) -> Result<()>
    where
        I: Iterator<Item = String>,
{
    let records = tags
        .map(|tag_name| NewArticleTag {
            article_id: id,
            tag_name,
        }).collect::<Vec<_>>();

    diesel::insert_into(article_tags::table)
        .values(&records)
        .on_conflict((article_tags::article_id, article_tags::tag_name))
        .do_nothing()
        .execute(conn)?;

    Ok(())
}

pub fn delete_tags(conn: &Conn, id: i32) -> Result<()> {
    let q = article_tags::table.filter(article_tags::article_id.eq(id));
    diesel::delete(q).execute(conn)?;
    Ok(())
}
