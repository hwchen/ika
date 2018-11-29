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

use crate::handlers::Agg;

pub struct PgExecutor(pub Pool<PostgresConnectionManager>);

impl Actor for PgExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Debug)]
pub struct PgQuery {
    pub schema: String,
    pub table: String,
    pub select: String,
    pub group_by: usize,
    pub limit: u16,
    pub agg: Agg,
}

impl Message for PgQuery {
    type Result = Result<String, Error>;
}

impl Handler<PgQuery> for PgExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: PgQuery, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;

        let query = format!("SELECT {} FROM {}.{} LIMIT {}",
            msg.select,
            msg.schema,
            msg.table,
            msg.limit
        );

        info!("query: {:?}, {:?}", query, msg);

        let rows = &conn.query(&query, &[])?;

        // set up the columns for "dataframe"
        let cols_meta = rows.columns();
        let mut df: Vec<ValueColumn> = vec![];

        for col_meta in cols_meta {
            match col_meta.type_().name() {
                "text" => df.push(ValueColumn::Text(vec![])),
                "int4" => df.push(ValueColumn::Int4(vec![])),
                "int2" => df.push(ValueColumn::Int2(vec![])),
                name => info!("type name: {}", name),
            }
        }

        // populate the "dataframe"
        for row in rows {
            for col_idx in 0..cols_meta.len() {
                match df.get_mut(col_idx).expect("logic checked") {
                    ValueColumn::Text(ss) => ss.push(row.get(col_idx)),
                    ValueColumn::Int4(ns) => ns.push(row.get(col_idx)),
                    ValueColumn::Int2(ns) => ns.push(row.get(col_idx)),
                }
            }
        }

        // TODO group by.

        info!("{:?}", df);

        Ok("".to_owned())
    }
}

#[derive(Debug)]
pub enum ValueColumn {
    Text(Vec<String>),
    Int4(Vec<i32>),
    Int2(Vec<i16>)
}
