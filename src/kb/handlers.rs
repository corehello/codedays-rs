// use super::AppState;
use actix_web::{
    web::Data, web::Json, web::Path, web::Query, HttpRequest, HttpResponse, Responder,
};
use validator::{Validate, ValidationError};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}


#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct NewTag {
    #[validate(length(min = 1, max = 64))]
    key: String,
    #[validate(length(min = 1, max = 64))]
    name: String,
}


pub async fn create_tag((form, req): (Json<NewTag>, HttpRequest)) -> impl Responder {
    let new_tag = form.into_inner();
    info!("cool");
    match new_tag.validate() {
        Ok(res) => return HttpResponse::Ok().json(new_tag),
        Err(e) => return HttpResponse::Ok().body("error"),
    }
}
