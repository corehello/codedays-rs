use super::handlers::index;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // 示例问题
    cfg.service(web::resource("/sample_problem").route(web::get().to(index)))
        // 问题列表
        .service(
            web::resource("/problems")
                .route(web::get().to(index))
                .route(web::post().to(index)),
        )
        // 单个问题
        .service(
            web::scope("/problems")
                .route("/{id}", web::get().to(index))
                .route("/{id}", web::delete().to(index)),
        );
}
