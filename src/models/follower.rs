use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::followers;

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(user_id, follower_id)]
pub struct Follower {
    pub user_id: Uuid,
    pub follower_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "followers"]
pub struct NewFollower {
    pub user_id: Uuid,
    pub follower_id: Uuid,
}
