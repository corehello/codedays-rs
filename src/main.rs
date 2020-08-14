#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate validator_derive;
extern crate validator;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;

mod config;
mod db;
mod error;
mod kb;
mod prelude;
mod schema;
mod subscriber;

use crate::config::Config;
use crate::db::{new_pool, DbExecutor};
use actix::prelude::{Addr, SyncArbiter};
use actix_web::{middleware, web, web::Data, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;

#[derive(Clone)]
pub struct AppState {
    pub db: Addr<DbExecutor>,
}

async fn index() -> impl Responder {
    info!("Hello world");
    HttpResponse::Ok().body("Hello World")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Config::from_env().unwrap();

    let database_pool = new_pool(config.database_url).expect("Failed to create pool.");
    // let database_address =
    //     SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

    let mut listenfd = ListenFd::from_env();
    debug!("Starting server at http://");
    // let state = AppState {
    //     db: database_address.clone(),
    // };
    let mut server = HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::scope("/api/v1")
                    .configure(kb::urls::configure)
                    .configure(subscriber::urls::configure),
            )
    });
    // 自动加载
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind(format!("{}:{}", config.server.host, config.server.port))?
    };

    server.run().await
}
