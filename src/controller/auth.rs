use crate::{
  core::{
    encryption::{encrypt_password, verify_password},
    jwt::Claims,
    mail::{mail_html, send_mail},
    settings::get_setting,
  },
  model::{
    auth::{
      RequestResetPasswrodRequest, ResetPasswrodRequest, SignInWithPasswordRequest,
      SignUpWithPasswordRequest,
    },
    error::ErrorResponse,
    user::User,
  },
  service::user::request_user_password_reset,
  service::user::{
    create_user, get_user_by_reset_token, get_user_by_username_or_email, reset_user_password,
    CreateUser,
  },
};
use actix_web::{
  post,
  web::{Data, ServiceConfig},
  HttpResponse, Responder,
};
use actix_web_validator::Json;
use anyhow::Result;
use lettre::{message::header::ContentType, Message};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[utoipa::path(
    request_body = SignInWithPasswordRequest,
    responses(
        (status = 200, description = "Sign's user in and returns JWT", content_type = "text/plain", body = String),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse),
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
      return HttpResponse::BadRequest().json(ErrorResponse::from("invalid_credentials"));
    }
  };

  match verify_password(&body.password, &user.encrypted_password) {
    Ok(true) => (),
    Ok(false) => {
      return HttpResponse::Unauthorized().json(ErrorResponse::from("invalid_credentials"))
    }
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(ErrorResponse::internal_error());
    }
  }

  let jwt = match Claims::new_encoded(pool.as_ref(), user.id).await {
    Ok(jwt) => jwt,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
    }
  };
  HttpResponse::Ok().content_type("text/plain").body(jwt)
}

#[utoipa::path(
    request_body = SignUpWithPasswordRequest,
    responses(
        (status = 201, description = "Create a new User and returns JWT", content_type = "text/plain", body = String),
        (status = 400, body = ErrorResponse),
        (status = 403, body = ErrorResponse),
        (status = 500, body = ErrorResponse),
    )
)]
#[post("/auth/sign-up/password")]
pub async fn sign_up_with_password(
  pool: Data<Pool<Postgres>>,
  body: Json<SignUpWithPasswordRequest>,
) -> impl Responder {
  if get_setting(pool.as_ref(), "disable_public_signup").await != serde_json::Value::Bool(false) {
    return HttpResponse::BadRequest().json(ErrorResponse::from("sign_up_disabled"));
  }
  if body.password != body.password_confirmation {
    return HttpResponse::BadRequest()
      .json(ErrorResponse::new().error("password_confirmation", "password_confirmation_mismatch"));
  }
  let default_role_id = match get_setting(pool.as_ref(), "default_role_id").await.as_i64() {
    Some(r) => r as i32,
    None => {
      log::error!("Missing settings for default_role_id");
      return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
    }
  };

  let encrypted_password_result = match encrypt_password(&body.password) {
    Ok(r) => r,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
    }
  };

  let (user, email) = match create_user(
    pool.as_ref(),
    CreateUser {
      role_id: default_role_id,
      username: body.username.to_owned(),
      email: body.email.clone(),
      encrypted_password: encrypted_password_result,
    },
  )
  .await
  {
    Ok(r) => r,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
    }
  };

  if let Some(email) = email.as_ref() {
    if let Some(confirmation_token) = email.confirmation_token.as_ref() {
      match create_confirmation_mail(&confirmation_token, &user.username, &email.email) {
        Ok(_) => {}
        Err(e) => {
          log::error!("{}", e);
          return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
        }
      }
    } else {
      log::error!("No confirmation token created");
      return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
    }
  }

  let jwt = match Claims::new_encoded(pool.as_ref(), user.id).await {
    Ok(jwt) => jwt,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
    }
  };
  HttpResponse::Created().content_type("text/plain").body(jwt)
}

#[utoipa::path(
    request_body = RequestResetPasswrodRequest,
    responses(
        (status = 204, description = "Reset password token created"),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse),
    )
)]
#[post("/auth/request-reset-password")]
pub async fn request_reset_password(
  pool: Data<Pool<Postgres>>,
  body: Json<RequestResetPasswrodRequest>,
) -> impl Responder {
  let (user, reset_password_token) =
    match request_user_password_reset(pool.as_ref(), &body.email).await {
      Ok(r) => r,
      Err(e) => {
        log::error!("{}", e);
        return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
      }
    };
  send_mail(pool.as_ref(), || {
    create_reset_password_mail(&user, &reset_password_token, &body.email)
  })
  .await;
  HttpResponse::NoContent().finish()
}

#[utoipa::path(
    request_body = ResetPasswrodRequest,
    responses(
        (status = 200, description = "Resets User's password", content_type = "text/plain", body = String),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse),
    )
)]
#[post("/auth/reset-password")]
pub async fn reset_password(
  pool: Data<Pool<Postgres>>,
  body: Json<ResetPasswrodRequest>,
) -> impl Responder {
  if body.password != body.password_confirmation {
    return HttpResponse::BadRequest().json(ErrorResponse::from("password_confirmation_mismatch"));
  }
  let user = match get_user_by_reset_token(pool.as_ref(), &body.reset_password_token).await {
    Ok(a) => a,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(ErrorResponse::from("invalid_reset_token"));
    }
  };
  let encrypted_password_result = match encrypt_password(&body.password) {
    Ok(r) => r,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
    }
  };
  match reset_user_password(pool.as_ref(), user.id, &encrypted_password_result).await {
    Ok(_) => {}
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
    }
  };

  let jwt = match Claims::new_encoded(pool.as_ref(), user.id).await {
    Ok(jwt) => jwt,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(ErrorResponse::internal_error());
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

fn create_confirmation_mail(
  confirmation_token: &Uuid,
  username: &str,
  email: &str,
) -> Result<Message> {
  let msg = Message::builder()
    .from("Support <support@aicacia.com>".parse()?)
    .to(format!("{} <{}>", username, email).parse()?)
    .subject("Confirmation Token")
    .header(ContentType::TEXT_HTML)
    .body(mail_html(format!(
      r#"
            <h1>Welcome!</h1>
            <p>Your confirmation token is: <code>{}</code></p>
        "#,
      confirmation_token.to_string()
    )))?;
  Ok(msg)
}

fn create_reset_password_mail(
  user: &User,
  reset_password_token: &Uuid,
  email: &str,
) -> Result<Message> {
  let msg = Message::builder()
    .from("Support <support@aicacia.com>".parse()?)
    .to(format!("{} <{}>", user.username, email).parse()?)
    .subject("Reset Password Request")
    .header(ContentType::TEXT_HTML)
    .body(mail_html(format!(
      r#"
            <h1>A Request to reset your password was made.</h1>
            <p>Your password reset token is: <code>{}</code></p>
        "#,
      reset_password_token
    )))?;
  Ok(msg)
}
