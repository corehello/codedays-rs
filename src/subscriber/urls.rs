use super::handlers::index;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/subscribe").route(web::post().to(index)))
        .service(web::resource("/unsubscribe").route(web::post().to(index)))
        .service(web::scope("/subscribe/confirm").route("/{id}", web::post().to(index)));
}
