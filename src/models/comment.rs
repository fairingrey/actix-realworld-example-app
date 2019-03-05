use chrono::NaiveDateTime;

use crate::schema::comments;

#[derive(Debug, Queryable)]
pub struct Comment {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub article_id: i32,
    pub user_id: i32,
    pub body: String,
}

#[derive(Debug, Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    pub article_id: i32,
    pub user_id: i32,
    pub body: String,
}
