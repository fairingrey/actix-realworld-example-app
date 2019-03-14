use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::comments;

#[derive(Debug, Queryable, Identifiable)]
pub struct Comment {
    pub id: i32,
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    pub article_id: Uuid,
    pub user_id: Uuid,
    pub body: String,
}
