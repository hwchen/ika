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
}

use actix_web::{
    HttpRequest,
    HttpResponse,
    Result as ActixResult,
};
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
