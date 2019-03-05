use crate::schema::followers;

#[derive(Debug, Insertable)]
#[table_name = "followers"]
pub struct NewFollower {
    pub user_id: i32,
    pub follower_id: i32,
}
