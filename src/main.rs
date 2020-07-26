#[macro_use]
extern crate diesel;

mod config;
mod kb;
mod schema;
mod subscriber;

use crate::config::Config;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;
use slog;
use slog::{debug, info};

#[derive(Clone)]
pub struct AppState {
    pub log: slog::Logger,
}

async fn index() -> impl Responder {
    let log = Config::configure_log();
    info!(log, "Hello world");
    HttpResponse::Ok().body("Hello World")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();
    let mut listenfd = ListenFd::from_env();
    let log = Config::configure_log();
    debug!(log, "Starting server at http://");
    let mut server = HttpServer::new(move || {
        App::new()
            .data(AppState { log: log.clone() })
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
