use crate::{
  core::{
    encryption::{encrypt_password, verify_password},
    jwt::Claims,
  },
  model::{
    auth::{
      RequestResetPasswordRequest, ResetPasswordRequest, SignInWithPasswordRequest,
      SignUpWithPasswordRequest,
    },
    error::Error,
  },
  service::{
    application::{can_sign_up_for_application, get_application_uri},
    user::{
      request_user_password_reset, user_email_taken, user_has_application, user_username_taken,
    },
  },
  service::{
    application::{get_application_jwt_expires_in_seconds, get_application_jwt_secret},
    user::{
      create_user, get_user_by_reset_token, get_user_by_username_or_email, reset_user_password,
      CreateUser,
    },
  },
};
use actix_web::{
  post,
  web::{Data, ServiceConfig},
  HttpResponse, Responder,
};
use actix_web_validator::Json;
use sqlx::{Pool, Postgres};

#[utoipa::path(
    request_body = SignInWithPasswordRequest,
    responses(
        (status = 200, description = "Sign's user in and returns JWT", content_type = "text/plain", body = String),
        (status = 400, body = Error),
        (status = 500, body = Error),
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
      return HttpResponse::BadRequest().json(Error::from("invalid_credentials"));
    }
  };

  match verify_password(&body.password, &user.encrypted_password) {
    Ok(true) => (),
    Ok(false) => return HttpResponse::Unauthorized().json(Error::from("invalid_credentials")),
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(Error::internal_error());
    }
  }

  match user_has_application(pool.as_ref(), user.id, body.application_id).await {
    Ok(true) => (),
    Ok(false) => {
      return HttpResponse::Unauthorized().json(Error::from("user_not_authorized_for_application"))
    }
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(Error::internal_error());
    }
  }

  let now_in_seconds = chrono::Utc::now().timestamp() as usize;
  let expires_in_seconds =
    get_application_jwt_expires_in_seconds(pool.as_ref(), body.application_id).await;
  let iss = get_application_uri(pool.as_ref(), body.application_id).await;
  let secret = get_application_jwt_secret(pool.as_ref(), body.application_id).await;
  let jwt = match Claims::new_encoded(
    body.application_id,
    user.id,
    now_in_seconds,
    expires_in_seconds,
    &iss,
    &secret,
  ) {
    Ok(jwt) => jwt,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Error::internal_error());
    }
  };
  HttpResponse::Ok().content_type("text/plain").body(jwt)
}

#[utoipa::path(
    request_body = SignUpWithPasswordRequest,
    responses(
        (status = 201, description = "Create a new User and returns JWT", content_type = "text/plain", body = String),
        (status = 400, body = Error),
        (status = 403, body = Error),
        (status = 500, body = Error),
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
      return HttpResponse::InternalServerError().json(Error::internal_error());
    }
  };
  if can_sign_up_for_application(pool.as_ref(), body.application_id).await {
    return HttpResponse::BadRequest().json(Error::from("sign_up_disabled"));
  }
  if body.password != body.password_confirmation {
    return HttpResponse::BadRequest()
      .json(Error::new().error("password_confirmation", "password_confirmation_mismatch"));
  }
  if let Some(email) = body.email.as_ref() {
    match user_email_taken(pool.as_ref(), email).await {
      Ok(false) => (),
      Ok(true) => {
        return HttpResponse::InternalServerError().json(Error::new().error("email", "taken"));
      }
      Err(e) => {
        log::error!("{}", e);
        return HttpResponse::InternalServerError().json(Error::internal_error());
      }
    };
  }
  match user_username_taken(pool.as_ref(), &body.username).await {
    Ok(false) => (),
    Ok(true) => {
      return HttpResponse::InternalServerError().json(Error::new().error("username", "taken"));
    }
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Error::internal_error());
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
      return HttpResponse::InternalServerError().json(Error::internal_error());
    }
  };

  let now_in_seconds = chrono::Utc::now().timestamp() as usize;
  let expires_in_seconds =
    get_application_jwt_expires_in_seconds(pool.as_ref(), body.application_id).await;
  let iss = get_application_uri(pool.as_ref(), body.application_id).await;
  let secret = get_application_jwt_secret(pool.as_ref(), body.application_id).await;
  let jwt = match Claims::new_encoded(
    body.application_id,
    user.id,
    now_in_seconds,
    expires_in_seconds,
    &iss,
    &secret,
  ) {
    Ok(jwt) => jwt,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Error::internal_error());
    }
  };
  HttpResponse::Created().content_type("text/plain").body(jwt)
}

#[utoipa::path(
    request_body = RequestResetPasswordRequest,
    responses(
        (status = 204, description = "Reset password token created"),
        (status = 400, body = Error),
        (status = 500, body = Error),
    )
)]
#[post("/auth/request-reset-password")]
pub async fn request_reset_password(
  pool: Data<Pool<Postgres>>,
  body: Json<RequestResetPasswordRequest>,
) -> impl Responder {
  let (_user, _reset_password_token) =
    match request_user_password_reset(pool.as_ref(), body.application_id, &body.email).await {
      Ok(r) => r,
      Err(e) => {
        log::error!("{}", e);
        return HttpResponse::InternalServerError().json(Error::internal_error());
      }
    };
  HttpResponse::NoContent().finish()
}

#[utoipa::path(
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Resets User's password", content_type = "text/plain", body = String),
        (status = 400, body = Error),
        (status = 500, body = Error),
    )
)]
#[post("/auth/reset-password")]
pub async fn reset_password(
  pool: Data<Pool<Postgres>>,
  body: Json<ResetPasswordRequest>,
) -> impl Responder {
  if body.password != body.password_confirmation {
    return HttpResponse::BadRequest().json(Error::from("password_confirmation_mismatch"));
  }
  let user = match get_user_by_reset_token(pool.as_ref(), &body.reset_password_token).await {
    Ok(a) => a,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest()
        .json(Error::new().error("reset_password_token", "invalid"));
    }
  };
  let encrypted_password_result = match encrypt_password(&body.password) {
    Ok(r) => r,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Error::internal_error());
    }
  };
  match reset_user_password(pool.as_ref(), user.id, &encrypted_password_result).await {
    Ok(_) => {}
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Error::internal_error());
    }
  };

  let now_in_seconds = chrono::Utc::now().timestamp() as usize;
  let expires_in_seconds =
    get_application_jwt_expires_in_seconds(pool.as_ref(), body.application_id).await;
  let iss = get_application_uri(pool.as_ref(), body.application_id).await;
  let secret = get_application_jwt_secret(pool.as_ref(), body.application_id).await;
  let jwt = match Claims::new_encoded(
    body.application_id,
    user.id,
    now_in_seconds,
    expires_in_seconds,
    &iss,
    &secret,
  ) {
    Ok(jwt) => jwt,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Error::internal_error());
    }
  };
  HttpResponse::Ok().content_type("text/plain").body(jwt)
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config
      .service(sign_in_with_password)
      .service(sign_up_with_password)
      .service(request_reset_password)
      .service(reset_password);
  }
}
