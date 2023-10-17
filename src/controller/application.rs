use actix_web::{
  get,
  web::{scope, Data, ServiceConfig},
  HttpResponse, Responder,
};
use sqlx::{Pool, Postgres};

use crate::{
  middleware::auth::Authorization,
  model::{application::Application, error::Errors},
  service::application::get_applications,
};

#[utoipa::path(
  context_path = "/applications",
  responses(
    (status = 200, description = "Get all application", body = Application),
    (status = 500, body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
#[get("")]
pub async fn index(pool: Data<Pool<Postgres>>) -> impl Responder {
  let applications = match get_applications(pool.as_ref()).await {
    Ok(e) => e,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(Errors::internal_error());
    }
  };
  let applications_response: Vec<Application> = applications.into_iter().map(Into::into).collect();
  HttpResponse::Ok().json(applications_response)
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config.service(scope("/applications").wrap(Authorization).service(index));
  }
}
