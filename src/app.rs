use actix::Addr;
use actix_web::{
    http::Method,
    middleware,
    App,
};
use serde_derive::Serialize;

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
    Result as ActixResult,
    State,
};
use futures::future::Future;
use log::*;

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
    (state, schema_table): (State<AppState>, Path<(String, String)>)
    ) -> FutureResponse<HttpResponse>
{
    let (schema, table) = schema_table.into_inner();
    info!("schema: {}, table: {}", schema, table);
    use crate::pg::PgTest;

    state
        .db
        .send(PgTest{ schema, table })
        .from_err()
        .and_then(|db_response| {
            match db_response {
                Ok(n) => Ok(HttpResponse::Ok().json(n)),
                Err(err) => Ok(HttpResponse::NotFound().json(err.to_string())),
            }
        })
        .responder()
}
