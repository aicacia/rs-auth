use actix_web::{
  get,
  web::{Data, ServiceConfig},
  HttpResponse, Responder,
};
use utoipa::openapi::OpenApi;

#[utoipa::path(
  responses(
      (status = 200, description = "OpenApi documenation", body = OpenApi),
  )
)]
#[get("/openapi.json")]
pub async fn openapi_json(openapi: Data<OpenApi>) -> impl Responder {
  HttpResponse::Ok().json(openapi)
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config.service(openapi_json);
  }
}
