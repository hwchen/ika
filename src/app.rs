use actix::Addr;
use actix_web::{
    http::Method,
    middleware,
    App,
};

use crate::handlers::{
    index_handler,
    test_handler,
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

