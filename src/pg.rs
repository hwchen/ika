use ::actix::{
    Actor,
    Handler,
    Message,
    SyncContext,
};
use ::r2d2::Pool;
use ::r2d2_postgres::PostgresConnectionManager;
use ::postgres::Connection as PgConnection;

pub struct PgExecutor(pub Pool<PostgresConnectionManager>);

impl Actor for PgExecutor {
    type Context = SyncContext<Self>;
}
