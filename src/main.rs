#[macro_use]
extern crate diesel;

mod kb;
mod schema;
mod subscriber;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let mut server = HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .configure(kb::urls::configure)
            .configure(subscriber::urls::configure)
    });
    // 自动加载
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8000")?
    };

    server.run().await
}
