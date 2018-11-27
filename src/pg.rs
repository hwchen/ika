use actix::{
    Actor,
    Handler,
    Message,
    SyncContext,
};
use failure::Error;
use log::*;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

pub struct PgExecutor(pub Pool<PostgresConnectionManager>);

impl Actor for PgExecutor {
    type Context = SyncContext<Self>;
}

pub struct PgTest {
    pub schema: String,
    pub table: String,
}

impl Message for PgTest {
    type Result = Result<i32, Error>;
}

impl Handler<PgTest> for PgExecutor {
    type Result = Result<i32, Error>;

    fn handle(&mut self, msg: PgTest, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;

        let query = format!("select st, agep from {}.{} limit 5",
            msg.schema,
            msg.table,
        );

        let res = &conn.query(&query, &[])?;

        let mut agep: i32 = 0;
        for row in res {
            info!("{:?}", row);
            agep = row.get(1);
            info!("{:?}", agep);
        }

        Ok(agep)
    }
}
