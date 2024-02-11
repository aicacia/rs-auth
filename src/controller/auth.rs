use crate::{
  core::{
    config::get_config,
    encryption::{encrypt_password, verify_password},
    jwt::{encode_jwt, Claims},
  },
  model::{
    auth::{
      RequestResetPasswordRequest, ResetPasswordRequest, SignInWithPasswordRequest, SignUpMethods,
      SignUpWithPasswordRequest,
    },
    error::Errors,
  },
  service::{
    application::{
      get_application_config, get_application_jwt_expires_in_seconds, get_application_jwt_secret,
    },
    user::{
      confirm_user_email, create_user, get_user_by_reset_token, get_user_by_username_or_email,
      request_user_password_reset, reset_user_password, user_email_taken, user_has_application,
      user_username_taken, CreateUser,
    },
  },
};
use actix_web::{
  get, post, put,
  web::{Data, Path, ServiceConfig},
  HttpResponse, Responder,
};
use actix_web_validator::Json;
use futures::join;
use sqlx::{Pool, Postgres};

#[utoipa::path(
    request_body = SignInWithPasswordRequest,
    responses(
        (status = 200, description = "Sign's user in and returns JWT", content_type = "text/plain", body = String),
        (status = 400, body = Errors),
        (status = 500, body = Errors),
    )
)]
#[post("/auth/sign-in/password")]
pub async fn sign_in_with_password(
  pool: Data<Pool<Postgres>>,
  body: Json<SignInWithPasswordRequest>,
) -> impl Responder {
  let user = match get_user_by_username_or_email(pool.as_ref(), &body.username_or_email).await {
    Ok(u) => u,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::Unauthorized().json(Errors::from("invalid_credentials"));
    }
  };

  match verify_password(&body.password, &user.encrypted_password) {
    Ok(true) => (),
    Ok(false) => return HttpResponse::Unauthorized().json(Errors::from("invalid_credentials")),
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  }

  match user_has_application(pool.as_ref(), body.application_id, user.id).await {
    Ok(true) => (),
    Ok(false) => {
      return HttpResponse::Unauthorized().json(Errors::from("user_not_authorized_for_application"))
    }
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  }

  let now_in_seconds = chrono::Utc::now().timestamp() as usize;
  let (expires_in_seconds, secret) = join!(
    get_application_jwt_expires_in_seconds(pool.as_ref(), body.application_id),
    get_application_jwt_secret(pool.as_ref(), body.application_id)
  );
  let config = get_config();
  let iss = config
    .server
    .uri
    .as_ref()
    .map(String::as_str)
    .unwrap_or("Auth");
  let jwt = match encode_jwt(
    &Claims::new(
      body.application_id,
      user.id,
      now_in_seconds,
      expires_in_seconds,
      iss,
    ),
    &secret,
  ) {
    Ok(jwt) => jwt,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  HttpResponse::Ok().content_type("text/plain").body(jwt)
}

#[utoipa::path(
    request_body = SignUpWithPasswordRequest,
    responses(
        (status = 201, description = "Create a new User and returns JWT", content_type = "text/plain", body = String),
        (status = 400, body = Errors),
        (status = 403, body = Errors),
        (status = 500, body = Errors),
    )
)]
#[post("/auth/sign-up/password")]
pub async fn sign_up_with_password(
  pool: Data<Pool<Postgres>>,
  body: Json<SignUpWithPasswordRequest>,
) -> impl Responder {
  let encrypted_password_result = match encrypt_password(&body.password) {
    Ok(r) => r,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  if get_application_config(pool.as_ref(), body.application_id, "signup.enabled").await
    != serde_json::Value::Bool(true)
  {
    return HttpResponse::BadRequest().json(Errors::from("sign_up_disabled"));
  }
  if get_application_config(pool.as_ref(), body.application_id, "signup.password").await
    != serde_json::Value::Bool(true)
  {
    return HttpResponse::BadRequest().json(Errors::from("password_sign_up_disabled"));
  }
  if body.password != body.password_confirmation {
    return HttpResponse::BadRequest()
      .json(Errors::new().error("password_confirmation", "password_confirmation_mismatch"));
  }
  if let Some(email) = body.email.as_ref() {
    match user_email_taken(pool.as_ref(), email).await {
      Ok(false) => (),
      Ok(true) => {
        return HttpResponse::InternalServerError().json(Errors::new().error("email", "taken"));
      }
      Err(e) => {
        log::error!("{}", e);
        return HttpResponse::InternalServerError().json(Errors::internal_error());
      }
    };
  }
  match user_username_taken(pool.as_ref(), &body.username).await {
    Ok(false) => (),
    Ok(true) => {
      return HttpResponse::InternalServerError().json(Errors::new().error("username", "taken"));
    }
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };

  let (user, _email) = match create_user(
    pool.as_ref(),
    body.application_id,
    CreateUser {
      username: body.username.to_owned(),
      email: body.email.clone(),
      encrypted_password: encrypted_password_result,
      send_confirmation_token: true,
    },
  )
  .await
  {
    Ok(r) => r,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };

  let now_in_seconds = chrono::Utc::now().timestamp() as usize;
  let (expires_in_seconds, secret) = join!(
    get_application_jwt_expires_in_seconds(pool.as_ref(), body.application_id),
    get_application_jwt_secret(pool.as_ref(), body.application_id)
  );
  let config = get_config();
  let iss = config
    .server
    .uri
    .as_ref()
    .map(String::as_str)
    .unwrap_or("Auth");
  let jwt = match encode_jwt(
    &Claims::new(
      body.application_id,
      user.id,
      now_in_seconds,
      expires_in_seconds,
      iss,
    ),
    &secret,
  ) {
    Ok(jwt) => jwt,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  HttpResponse::Created().content_type("text/plain").body(jwt)
}

#[utoipa::path(
    request_body = RequestResetPasswordRequest,
    responses(
        (status = 204, description = "Reset password token created"),
        (status = 400, body = Errors),
        (status = 500, body = Errors),
    )
)]
#[post("/auth/request-reset-password")]
pub async fn request_reset_password(
  pool: Data<Pool<Postgres>>,
  body: Json<RequestResetPasswordRequest>,
) -> impl Responder {
  let (_user, _reset_password_token) =
    match request_user_password_reset(pool.as_ref(), body.application_id, &body.username_or_email)
      .await
    {
      Ok(r) => r,
      Err(e) => {
        log::error!("{}", e);
        return HttpResponse::BadRequest().json(Errors::bad_request());
      }
    };
  HttpResponse::NoContent().finish()
}

#[utoipa::path(
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Resets User's password", content_type = "text/plain", body = String),
        (status = 400, body = Errors),
        (status = 500, body = Errors),
    )
)]
#[post("/auth/reset-password")]
pub async fn reset_password_with_token(
  pool: Data<Pool<Postgres>>,
  body: Json<ResetPasswordRequest>,
) -> impl Responder {
  if body.password != body.password_confirmation {
    return HttpResponse::BadRequest().json(Errors::from("password_confirmation_mismatch"));
  }
  let user = match get_user_by_reset_token(pool.as_ref(), &body.reset_password_token).await {
    Ok(a) => a,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest()
        .json(Errors::new().error("reset_password_token", "invalid"));
    }
  };
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

  let now_in_seconds = chrono::Utc::now().timestamp() as usize;
  let (expires_in_seconds, secret) = join!(
    get_application_jwt_expires_in_seconds(pool.as_ref(), body.application_id),
    get_application_jwt_secret(pool.as_ref(), body.application_id)
  );
  let config = get_config();
  let iss = config
    .server
    .uri
    .as_ref()
    .map(String::as_str)
    .unwrap_or("Auth");
  let jwt = match encode_jwt(
    &Claims::new(
      body.application_id,
      user.id,
      now_in_seconds,
      expires_in_seconds,
      iss,
    ),
    &secret,
  ) {
    Ok(jwt) => jwt,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  HttpResponse::Ok().content_type("text/plain").body(jwt)
}

#[utoipa::path(
  responses(
      (status = 204, description = "Confirms email with confirmation token"),
      (status = 400, body = Errors),
  )
)]
#[put("/auth/confirm-email/{confirmation_token}")]
pub async fn confirm_email(path: Path<uuid::Uuid>, pool: Data<Pool<Postgres>>) -> impl Responder {
  let confirmation_token = path.into_inner();
  match confirm_user_email(pool.as_ref(), &confirmation_token).await {
    Ok(true) => (),
    Ok(false) => {
      return HttpResponse::BadRequest().json(Errors::new().error("confirmation_token", "invalid"));
    }
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  HttpResponse::NoContent().finish()
}

#[utoipa::path(
  responses(
      (status = 200, description = "Returns sign up methods for an application", body = SignUpMethods),
      (status = 400, body = Errors),
  )
)]
#[get("/auth/sign-up-methods/{application_id}")]
pub async fn sign_up_methods(path: Path<i32>, pool: Data<Pool<Postgres>>) -> impl Responder {
  let application_id = path.into_inner();
  let mut sign_up_methods_response = SignUpMethods::default();

  let (signup_enabled, signup_password) = join!(
    async {
      get_application_config(pool.as_ref(), application_id, "signup.enabled")
        .await
        .as_bool()
        .unwrap_or(false)
    },
    async {
      get_application_config(pool.as_ref(), application_id, "signup.password")
        .await
        .as_bool()
        .unwrap_or(false)
    },
  );

  if signup_enabled {
    sign_up_methods_response.enabled = signup_enabled;
    sign_up_methods_response.password = signup_password;
  }

  HttpResponse::Ok().json(sign_up_methods_response.validate())
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config
      .service(sign_in_with_password)
      .service(sign_up_with_password)
      .service(request_reset_password)
      .service(reset_password_with_token)
      .service(confirm_email)
      .service(sign_up_methods);
  }
}
