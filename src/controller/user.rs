use actix_web::{
  get, put,
  web::{scope, Data, Path, ServiceConfig},
  HttpResponse, Responder,
};
use actix_web_validator::Json;
use futures::join;
use sqlx::{Pool, Postgres};

use crate::{
  core::{encryption::encrypt_password, jwt::Claims},
  middleware::auth::Authorization,
  model::{
    application::{Application, ApplicationRow},
    error::Errors,
    user::{ResetUserPasswordRequest, User, UserRow},
  },
  service::{
    application::{
      get_application_jwt_expires_in_seconds, get_application_jwt_secret, get_application_uri,
    },
    user::{get_user_applications, get_user_emails, reset_user_password, set_user_primary_email},
  },
};

#[utoipa::path(
  context_path = "/users",
  responses(
      (status = 200, description = "Get current user", body = User),
      (status = 500, body = Errors),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[get("/current")]
pub async fn current(pool: Data<Pool<Postgres>>, user: UserRow) -> impl Responder {
  let emails = match get_user_emails(pool.as_ref(), user.id).await {
    Ok(e) => e,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(Errors::internal_error());
    }
  };
  let user_response: User = (user, emails).into();
  HttpResponse::Ok().json(user_response)
}

#[utoipa::path(
  context_path = "/users",
  responses(
      (status = 200, description = "Get current user's application", body = Application),
      (status = 500, body = Errors),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[get("/applications")]
pub async fn applications(pool: Data<Pool<Postgres>>, user: UserRow) -> impl Responder {
  let applications = match get_user_applications(pool.as_ref(), user.id).await {
    Ok(e) => e,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(Errors::internal_error());
    }
  };
  let applications_response: Vec<Application> = applications.into_iter().map(Into::into).collect();
  HttpResponse::Ok().json(applications_response)
}

#[utoipa::path(
  context_path = "/users",
  responses(
      (status = 204, description = "Sets email as primary"),
      (status = 400, body = Errors),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[put("/set-primary-email/{email_id}")]
pub async fn set_primary_email(
  user: UserRow,
  path: Path<i32>,
  pool: Data<Pool<Postgres>>,
) -> impl Responder {
  let email_id = path.into_inner();
  match set_user_primary_email(pool.as_ref(), user.id, email_id).await {
    Ok(true) => (),
    Ok(false) => {
      return HttpResponse::BadRequest().json(Errors::new().error("email_id", "invalid_email"));
    }
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(Errors::internal_error());
    }
  };
  HttpResponse::NoContent().finish()
}

#[utoipa::path(
  context_path = "/users",
  request_body = ResetUserPasswordRequest,
  responses(
      (status = 200, description = "Resets User's password", content_type = "text/plain", body = String),
      (status = 400, body = Errors),
      (status = 500, body = Errors),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[put("/reset-password")]
pub async fn reset_password(
  pool: Data<Pool<Postgres>>,
  user: UserRow,
  body: Json<ResetUserPasswordRequest>,
) -> impl Responder {
  if body.password != body.password_confirmation {
    return HttpResponse::BadRequest().json(Errors::from("password_confirmation_mismatch"));
  }
  let encrypted_password_result = match encrypt_password(&body.password) {
    Ok(r) => r,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  match reset_user_password(pool.as_ref(), user.id, &encrypted_password_result).await {
    Ok(_) => {}
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  HttpResponse::NoContent().finish()
}

#[utoipa::path(
  context_path = "/users",
  responses(
      (status = 201, description = "Refreshes User's JWT", content_type = "text/plain", body = String),
      (status = 400, body = Errors),
      (status = 500, body = Errors),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[get("/refresh-token")]
pub async fn refresh_token(
  pool: Data<Pool<Postgres>>,
  application: ApplicationRow,
  user: UserRow,
) -> impl Responder {
  let now_in_seconds = chrono::Utc::now().timestamp() as usize;
  let (expires_in_seconds, iss, secret) = join!(
    get_application_jwt_expires_in_seconds(pool.as_ref(), application.id),
    get_application_uri(pool.as_ref(), application.id),
    get_application_jwt_secret(pool.as_ref(), application.id)
  );
  let jwt = match Claims::new(
    application.id,
    user.id,
    now_in_seconds,
    expires_in_seconds,
    &iss,
  )
  .encode(&secret)
  {
    Ok(jwt) => jwt,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  HttpResponse::Created().content_type("text/plain").body(jwt)
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config.service(
      scope("/users")
        .wrap(Authorization)
        .service(current)
        .service(set_primary_email)
        .service(refresh_token)
        .service(reset_password)
        .service(applications),
    );
  }
}
