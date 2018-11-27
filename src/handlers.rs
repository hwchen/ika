use actix_web::{
    AsyncResponder,
    FutureResponse,
    HttpRequest,
    HttpResponse,
    Path,
    Query,
    Result as ActixResult,
    State,
};
use futures::future::Future;
use log::*;
use serde_derive::{Serialize, Deserialize};

use crate::app::AppState;

pub fn index_handler(_req: HttpRequest<AppState>) -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(
        Status {
            status: "ok".to_owned(),
            version: "0.1.0".to_owned(),
        }
    ))
}

#[derive(Debug, Serialize)]
struct Status {
    status: String,
    version: String,
}

pub fn test_handler(
    (state, schema_table, query): (State<AppState>, Path<(String, String)>, Query<QueryOpt>)
    ) -> FutureResponse<HttpResponse>
{
    let (schema, table) = schema_table.into_inner();
    info!("schema: {}, table: {}", schema, table);
    use crate::pg::PgQuery;

    let (agg_type, col_idx) = match &query.agg.split('.').collect::<Vec<_>>()[..2] {
        &[agg_type, col_idx] => (agg_type, col_idx),
            _ => panic!("incorrect syntax for agg"),
    };

    let agg = match agg_type {
        "sum" => Agg::Sum(col_idx.parse().expect("not a num")),
        "count" => Agg::Count(col_idx.parse().expect("not a num")),
        _ => panic!("agg type not supported"),
    };

    let pg_query = PgQuery {
        schema,
        table,
        select: query.select.to_owned(),
        group_by: query.group_by,
        agg,
    };

    state
        .db
        .send(pg_query)
        .from_err()
        .and_then(|db_response| {
            match db_response {
                Ok(n) => Ok(HttpResponse::Ok().json(n)),
                Err(err) => Ok(HttpResponse::NotFound().json(err.to_string())),
            }
        })
        .responder()
}

#[derive(Debug, Deserialize)]
pub struct QueryOpt {
    select: String,
    group_by: usize,
    agg: String,
}

// only aggregating one column right now
#[derive(Debug, Deserialize)]
pub enum Agg{
    Sum(usize),
    Count(usize),
}


