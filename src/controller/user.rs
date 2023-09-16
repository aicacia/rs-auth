use actix_web::{
  get, put,
  web::{scope, Data, Path, ServiceConfig},
  HttpResponse, Responder,
};
use sqlx::{Pool, Postgres};

use crate::{
  middleware::auth::Authorization,
  model::{
    error::ErrorResponse,
    user::{User, UserResponse},
  },
  service::user::{confirm_user_email, get_user_emails, set_user_primary_email},
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

#[utoipa::path(
  context_path = "/users",
  responses(
      (status = 204, description = "Confirms email with confirmation token"),
      (status = 400, body = ErrorResponse),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[put("/confirm-email/{confirmation_token}")]
pub async fn confirm_email(
  user: User,
  path: Path<uuid::Uuid>,
  pool: Data<Pool<Postgres>>,
) -> impl Responder {
  let confirmation_token = path.into_inner();
  match confirm_user_email(pool.as_ref(), user.id, &confirmation_token).await {
    Ok(true) => (),
    Ok(false) => {
      return HttpResponse::BadRequest()
        .json(ErrorResponse::new().error("confirmation_token", "invalid_confirmation_token"));
    }
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(ErrorResponse::internal_error());
    }
  };
  HttpResponse::NoContent().finish()
}

#[utoipa::path(
  context_path = "/users",
  responses(
      (status = 204, description = "Sets email as primary"),
      (status = 400, body = ErrorResponse),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[put("/set-primary-email/{email_id}")]
pub async fn set_primary_email(
  user: User,
  path: Path<i32>,
  pool: Data<Pool<Postgres>>,
) -> impl Responder {
  let email_id = path.into_inner();
  match set_user_primary_email(pool.as_ref(), user.id, email_id).await {
    Ok(true) => (),
    Ok(false) => {
      return HttpResponse::BadRequest()
        .json(ErrorResponse::new().error("email_id", "invalid_email"));
    }
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(ErrorResponse::internal_error());
    }
  };
  HttpResponse::NoContent().finish()
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config.service(
      scope("/users")
        .wrap(Authorization)
        .service(current)
        .service(confirm_email)
        .service(set_primary_email),
    );
  }
}
