use actix::prelude::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{
    ConnectionManager,
    Pool
};

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
