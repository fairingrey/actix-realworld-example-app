mod articles;
mod auth;
mod comments;
mod profiles;
mod tags;
mod users;

use crate::prelude::*;
use actix::prelude::{Actor, SyncContext};
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
};

pub type Conn = PgConnection;
pub type PgPool = Pool<ConnectionManager<Conn>>;
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
