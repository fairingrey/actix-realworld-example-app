use uuid::Uuid;

use crate::schema::followers;

#[derive(Debug, Insertable)]
#[table_name = "followers"]
pub struct NewFollower {
    pub user_id: Uuid,
    pub follower_id: Uuid,
}
