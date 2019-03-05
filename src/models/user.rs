use chrono::NaiveDateTime;

use crate::schema::{credentials, users};

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[cfg(test)]
impl Default for NewUser {
    fn default() -> Self {
        NewUser {
            username: String::new(),
            email: String::new(),
            bio: None,
            image: None,
        }
    }
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub username: Option<String>,
    pub email: Option<String>,
    pub bio: Option<Option<String>>,
    pub image: Option<Option<String>>,
}

#[derive(Debug, Queryable)]
pub struct Credential {
    pub id: i32,
    pub user_id: i32,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "credentials"]
pub struct NewCredential {
    pub user_id: i32,
    pub password_hash: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "credentials"]
pub struct CredentialChange {
    pub password_hash: Option<String>,
}
