// Policy of these sub modules:
// - Define only common and simple operations such as insertion, deletion, etc.
// - Uncommon SELECT queries should be written in their services.

pub mod articles;
//pub mod comments;
//pub mod followers;
//pub mod users;

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
};

pub type Conn = diesel::pg::PgConnection;
pub type PgPool = r2d2::Pool<ConnectionManager<Conn>>;
pub type PooledConn = PooledConnection<ConnectionManager<Conn>>;


pub struct DbExecutor(pub PgPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn new_pool<S: Into<String>>(database_url: S) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<Conn>::new(database_url.into());
    r2d2::Pool::builder().build(manager)
}

pub fn get_conn(pool: &PgPool) -> Result<PooledConn, PoolError> {
    pool.get()
}

