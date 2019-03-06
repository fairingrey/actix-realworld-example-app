use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::comments;

#[derive(Debug, Queryable)]
pub struct Comment {
    pub id: usize,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub body: String,
}

#[derive(Debug, Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub body: String,
}
