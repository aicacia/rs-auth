use crate::model::util::{Health, HealthResponse, VersionResponse};
use actix_web::{
  get,
  web::{Data, ServiceConfig},
  HttpResponse, Responder,
};
use sqlx::{Pool, Postgres};

#[utoipa::path(
    responses(
        (status = 200, description = "Health check response", body = HealthResponse),
    )
)]
#[get("/health")]
pub async fn health(pool: Data<Pool<Postgres>>) -> impl Responder {
  let health = Health {
    db: pool.acquire().await.is_ok(),
  };

  let health_response: HealthResponse = health.into();
  if health_response.ok {
    HttpResponse::Ok().json(health_response)
  } else {
    HttpResponse::InternalServerError().json(health_response)
  }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Version response", body = VersionResponse),
    )
)]
#[get("/version")]
pub async fn version() -> impl Responder {
  HttpResponse::Ok().json(VersionResponse {
    version: env!("CARGO_PKG_VERSION").to_owned(),
  })
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config.service(health).service(version);
  }
}
