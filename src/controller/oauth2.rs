use actix_web::{
  get,
  web::{Data, ServiceConfig},
  HttpResponse, Responder,
};
use actix_web_validator::Query;
use sqlx::{Pool, Postgres};

use crate::model::oauth2::AuthorizeQuery;

#[utoipa::path(
  responses(
    (status = 201, description = "Created url"),
    (status = 500, body = Errors),
  )
)]
#[get("/oauth2/authorize")]
pub async fn authorize(pool: Data<Pool<Postgres>>, query: Query<AuthorizeQuery>) -> impl Responder {
  HttpResponse::NoContent().finish()
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config.service(authorize);
  }
}
