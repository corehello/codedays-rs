// use super::AppState;
use super::models::Tag;
use crate::db::{PgPool as Pool, PooledConn};
use crate::diesel::RunQueryDsl;
use crate::error::Error;
use crate::schema::tag;
use actix_web::{
    web::Data, web::Json, web::Path, web::Query, HttpRequest, HttpResponse, Responder,
};
use chrono::NaiveDateTime;
use diesel;
use validator::{Validate, ValidationErrors};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[derive(Debug, Validate, Deserialize)]
pub struct NewProblem {
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
    pub updated_time: NaiveDateTime,
    #[validate(range(min = 1, max = 3))]
    pub difficulty: i8,
    pub category: i32,
    // pub tags: i32,
    #[validate(range(min = 1))]
    pub order: i32,
}

#[derive(Debug, Validate, Deserialize)]
pub struct NewCategory {
    #[validate(length(min = 1, max = 64))]
    name: String,
}

#[derive(Debug, Validate, Deserialize, Serialize, Insertable)]
#[table_name = "tag"]
pub struct NewTag {
    #[validate(length(min = 1, max = 64))]
    key: String,
    #[validate(length(min = 1, max = 64))]
    name: String,
}

impl NewTag {
    pub fn create(&self, pool: Data<Pool>) -> Result<Tag, diesel::result::Error> {
        let conn = pool.get().expect("couldn't get db connection from pool");
        diesel::insert_into(tag::table)
            .values(self)
            .get_result(&conn)
    }
}

pub async fn create_tag(
    pool: Data<Pool>,
    (form, req): (Json<NewTag>, HttpRequest),
) -> Result<HttpResponse, HttpResponse> {
    info!("create_tag");
    let new_tag = form.into_inner();
    new_tag.validate().expect("no error");
    new_tag
        .create(pool)
        .map(|tag| HttpResponse::Ok().json(tag))
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
