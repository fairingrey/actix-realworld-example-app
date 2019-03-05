use crate::models::NewFollower;
use crate::prelude::*;
use crate::schema::followers;
use crate::db::Conn;
use diesel::prelude::*;
use std::iter::Iterator;

pub fn insert(conn: &Conn, new_follower: &NewFollower) -> Result<()> {
    diesel::insert_into(followers::table)
        .values(new_follower)
        .on_conflict((followers::user_id, followers::follower_id))
        .do_nothing()
        .execute(conn)?;

    Ok(())
}

pub fn delete(conn: &Conn, user_id: i32, follower_id: i32) -> Result<()> {
    diesel::delete(
        followers::table
            .filter(followers::user_id.eq(user_id))
            .filter(followers::follower_id.eq(follower_id)),
    ).execute(conn)?;

    Ok(())
}

pub fn filter_followee_ids(
    conn: &Conn,
    follower_id: i32,
    followee_ids: &[i32],
) -> Result<impl Iterator<Item = i32>> {
    let ids = followers::table
        .filter(followers::user_id.eq_any(followee_ids))
        .filter(followers::follower_id.eq(follower_id))
        .select(followers::user_id)
        .load::<i32>(conn)?;

    Ok(ids.into_iter())
}
