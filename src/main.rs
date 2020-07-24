use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
