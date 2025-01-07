use axum::{extract::State, response::IntoResponse};
use http::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  core::error::{Errors, ALREADY_EXISTS_ERROR, INTERNAL_ERROR, NOT_FOUND_ERROR},
  middleware::{json::Json, user_authorization::UserAuthorization},
  model::totp::{CreateTOTPRequest, UserTOTP},
  repository::user_totp::{create_user_totp, delete_user_totp, CreateUserTOTP},
};

use super::{current_user::CURRENT_USER_TAG, RouterState};

#[utoipa::path(
  post,
  path = "/current-user/totp",
  tags = [CURRENT_USER_TAG],
  request_body = CreateTOTPRequest,
  responses(
    (status = 201, content_type = "application/json", body = UserTOTP),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 409, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn create_current_user_totp(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
  Json(payload): Json<CreateTOTPRequest>,
) -> impl IntoResponse {
  let totp = match create_user_totp(
    &state.pool,
    user.id,
    CreateUserTOTP {
      secret: totp_rs::Secret::generate_secret().to_encoded().to_string(),
      algorithm: payload.algorithm.unwrap_or("SHA1".to_owned()),
      digits: payload.digits.unwrap_or(6),
      step: payload.step.unwrap_or(30),
    },
  )
  .await
  {
    Ok(totp) => totp,
    Err(e) => {
      if e.to_string().to_lowercase().contains("unique constraint") {
        return Errors::from(StatusCode::CONFLICT)
          .with_error("totp", ALREADY_EXISTS_ERROR)
          .into_response();
      }
      log::error!("Error creating user TOTP: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  (StatusCode::CREATED, axum::Json(UserTOTP::from(totp))).into_response()
}

#[utoipa::path(
  delete,
  path = "/current-user/totp",
  tags = [CURRENT_USER_TAG],
  responses(
    (status = 204),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn delete_current_user_totp(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
) -> impl IntoResponse {
  match delete_user_totp(&state.pool, user.id).await {
    Ok(Some(_)) => {}
    Ok(None) => {
      return Errors::not_found()
        .with_error("totp", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("Error creating user TOTP: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(create_current_user_totp, delete_current_user_totp))
    .with_state(state)
}
