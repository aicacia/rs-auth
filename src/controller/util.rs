use crate::model::util::{Health, Version};
use actix_web::{
  get,
  web::{Data, ServiceConfig},
  HttpResponse, Responder,
};
use sqlx::{Pool, Postgres};

#[utoipa::path(
    responses(
        (status = 200, description = "Health check response", body = Health),
    )
)]
#[get("/health")]
pub async fn health(pool: Data<Pool<Postgres>>) -> impl Responder {
  let health = Health {
    db: pool.acquire().await.is_ok(),
  };

  if health.is_healthy() {
    HttpResponse::Ok().json(health)
  } else {
    HttpResponse::InternalServerError().json(health)
  }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Version response", body = Version),
    )
)]
#[get("/version")]
pub async fn version() -> impl Responder {
  HttpResponse::Ok().json(Version {
    version: env!("CARGO_PKG_VERSION").to_owned(),
  })
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config.service(health).service(version);
  }
}
