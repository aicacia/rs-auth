use actix_web::{
  get,
  web::{scope, Data, ServiceConfig},
  HttpResponse, Responder,
};
use sqlx::{Pool, Postgres};

use crate::{
  middleware::auth::Authorization,
  model::{
    error::ErrorResponse,
    user::{User, UserResponse},
  },
  service::user::get_user_emails,
};

#[utoipa::path(
  context_path = "/users",
  responses(
      (status = 200, description = "Get current user", body = UserResponse),
      (status = 500, body = ErrorResponse),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[get("/current")]
pub async fn current(pool: Data<Pool<Postgres>>, user: User) -> impl Responder {
  let emails = match get_user_emails(pool.as_ref(), user.id).await {
    Ok(e) => e,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(ErrorResponse::internal_error());
    }
  };
  let user_response: UserResponse = (user, emails).into();
  HttpResponse::Ok().json(user_response)
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config.service(scope("/users").wrap(Authorization).service(current));
  }
}
