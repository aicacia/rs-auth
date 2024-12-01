use axum::{
  Router,
  extract::State,
  response::IntoResponse,
  routing::{delete, post},
};
use http::StatusCode;
use utoipa::OpenApi;

use crate::{
  core::error::{Errors, INTERNAL_ERROR, REQUIRED_ERROR},
  middleware::{json::Json, user_authorization::UserAuthorization},
  model::totp::{CreateTOTPRequest, UserTOTP},
  repository::user_totp::{CreateUserTOTP, create_user_totp, delete_user_totp},
};

use super::RouterState;

#[derive(OpenApi)]
#[openapi(
  paths(
    create_totp,
    delete_totp,
  ),
  components(
    schemas(
      CreateTOTPRequest
    )
  ),
  tags(
    (name = "totp", description = "TOTP endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  post,
  path = "current-user/totp",
  tags = ["current-user", "totp"],
  request_body = CreateTOTPRequest,
  responses(
    (status = 201, content_type = "application/json", body = UserTOTP),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("UserAuthorization" = [])
  )
)]
pub async fn create_totp(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
  Json(payload): Json<CreateTOTPRequest>,
) -> impl IntoResponse {
  let totp = match create_user_totp(&state.pool, user.id, CreateUserTOTP {
    secret: totp_rs::Secret::generate_secret().to_encoded().to_string(),
    algorithm: payload.algorithm.unwrap_or("SHA1".to_owned()),
    digits: payload.digits.unwrap_or(6),
    step: payload.step.unwrap_or(30),
  })
  .await
  {
    Ok(totp) => totp,
    Err(e) => {
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
  path = "current-user/totp",
  tags = ["current-user", "totp"],
  responses(
    (status = 204),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("UserAuthorization" = [])
  )
)]
pub async fn delete_totp(
  State(state): State<RouterState>,
  UserAuthorization { user, .. }: UserAuthorization,
) -> impl IntoResponse {
  match delete_user_totp(&state.pool, user.id).await {
    Ok(Some(_)) => {}
    Ok(None) => {
      return Errors::not_found()
        .with_error("totp", REQUIRED_ERROR)
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

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/current-user/totp", post(create_totp))
    .route("/current-user/totp", delete(delete_totp))
    .with_state(state)
}
