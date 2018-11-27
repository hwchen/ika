use actix::Addr;
use actix_web::{
    http::Method,
    middleware,
    App,
};

use crate::pg::PgExecutor;

pub struct AppState {
    pub db: Addr<PgExecutor>,
}

pub fn create_app(db: Addr<PgExecutor>) -> App<AppState> {
    App::with_state(AppState { db })
        .middleware(middleware::Logger::default())
        .resource("/", |r| {
            r.method(Method::GET).with(index_handler)
        })
        .resource("/test/{schema}/{table}", |r| {
            r.method(Method::GET).with(test_handler)
        })
}

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

    let pg_query = PgQuery {
        schema,
        table,
        select: query.select.to_owned(),
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
}
