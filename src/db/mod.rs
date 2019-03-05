// Policy of these sub modules:
// - Define only common and simple operations such as insertion, deletion, etc.
// - Uncommon SELECT queries should be written in their services.

pub mod articles;
pub mod comments;
pub mod followers;
pub mod users;

use actix::prelude::{Actor, SyncContext};
use diesel::{
    pg::PgConnection,
    r2d2::{
        self,
        ConnectionManager,
        Pool,
        PoolError,
        PooledConnection,
    },
    result::Error as DieselError,
};
use crate::prelude::*;

pub type Conn = diesel::pg::PgConnection;
pub type PgPool = r2d2::Pool<ConnectionManager<Conn>>;
pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;

pub struct DbExecutor(pub PgPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn new_pool<S: Into<String>>(database_url: S) -> Result<PgPool> {
    let manager = ConnectionManager::<Conn>::new(database_url.into());
    let pool = r2d2::Pool::builder().build(manager)?;
    Ok(pool)

}

pub fn get_conn(pool: &PgPool) -> Result<PooledConn> {
    let conn = pool.get()?;
    Ok(conn)
}

/// Ignores diesel's `QueryBuilderError` silently. This error could occur when
/// you run an update with a changeset whose all fields are `None`.
/// In that case, this returns `Ok(None)`.
pub fn may_update<T>(result: Result<T, DieselError>) -> Result<Option<T>, DieselError> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(err) => match err {
            DieselError::QueryBuilderError(_) => Ok(None),
            err => Err(err),
        },
    }
}

/// Return `Some(new)` if `new` is not equal to `old`, otherwise `None`.
/// This is useful to set up diesel's `AsChangeset` struct to update
/// only the changed columns.
pub fn if_changed<T: PartialEq>(new: Option<T>, old: &T) -> Option<T> {
    new.and_then(|new| if new != *old { Some(new) } else { None })
}
