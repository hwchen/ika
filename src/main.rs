//! ika
//!
//! A REST api, with routes generated from table metadata.
//! Explores the boundaries between pure REST and OLAP capabilities such as aggregation

use actix::{Addr, SyncArbiter};
use actix_web::server;
use dotenv::dotenv;
use failure::{Error, format_err};
use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use std::env;
use structopt::StructOpt;

mod app;
mod handlers;
mod pg;

use crate::pg::PgExecutor;

fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    // Configuration
    dotenv().ok();
    let opt = Opt::from_args();

    let server_addr = opt.address.unwrap_or("127.0.0.1:4000".to_owned());

    let pg_database_url = env::var("DATABASE_URL")
        .or(opt.database_url.ok_or(format_err!("")))
        .expect("No DATABASE_URL Found");

    let sys = actix::System::new("ika");

    // Initialize postgres
    let pg_manager = PostgresConnectionManager::new(pg_database_url, TlsMode::None)?;
    let pg_pool = r2d2::Pool::new(pg_manager)
        .expect("Failed to create Postgres DbPool");
    let pg_address: Addr<PgExecutor> = SyncArbiter::start(4, move || {
        PgExecutor(pg_pool.clone())
    });

    server::new(move|| app::create_app(pg_address.clone()))
        .bind(&server_addr)
        .expect(&format!("cannot find to {}", server_addr))
        .start();

    println!("Listening on {}", server_addr);

    sys.run();

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name="ika")]
struct Opt {
    #[structopt(short="a", long="addr")]
    address: Option<String>,
    #[structopt(long="database-url")]
    database_url: Option<String>,
}
