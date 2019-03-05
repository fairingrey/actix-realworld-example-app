use crate::db::Conn;
use crate::models::{Comment, NewComment};
use crate::prelude::*;
use crate::schema::comments;
use diesel::prelude::*;

pub fn insert(conn: &Conn, comment: &NewComment) -> Result<Comment> {
    diesel::insert_into(comments::table)
        .values(comment)
        .get_result::<Comment>(conn)
        .map_err(|e| e.into())
}

pub fn delete(conn: &Conn, id: i32) -> Result<()> {
    diesel::delete(comments::table.filter(comments::id.eq(id))).execute(conn)?;
    Ok(())
}
