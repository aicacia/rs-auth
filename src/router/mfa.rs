use axum::{Router, extract::State, response::IntoResponse, routing::post};
use utoipa::OpenApi;

use crate::{
  core::{
    error::{Errors, INTERNAL_ERROR, INVALID_ERROR, REQUIRED_ERROR},
    openapi::AUTHORIZATION_HEADER,
  },
  middleware::{
    authorization::Authorization,
    claims::{BasicClaims, TOKEN_TYPE_MFA_TOTP},
    json::Json,
  },
  model::{
    mfa::MFARequest,
    token::{TOKEN_ISSUED_TYPE_MFA, Token},
  },
  repository::{tenent::TenentRow, user::get_user_by_id, user_totp::get_user_totp_by_user_id},
};

use super::{RouterState, token::create_user_token};

#[derive(OpenApi)]
#[openapi(
  paths(
    mfa,
  ),
  components(
    schemas(
      MFARequest
    )
  ),
  tags(
    (name = "mfa", description = "Multi-factor authentication endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  post,
  path = "mfa",
  tags = ["mfa"],
  request_body = MFARequest,
  responses(
    (status = 201, content_type = "application/json", body = Token),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn mfa(
  State(state): State<RouterState>,
  Authorization { claims, tenent, .. }: Authorization,
  Json(payload): Json<MFARequest>,
) -> impl IntoResponse {
  match payload {
    MFARequest::TOTP { code } => totp_request(&state.pool, claims, tenent, code)
      .await
      .into_response(),
  }
}

async fn totp_request(
  pool: &sqlx::Pool<sqlx::Any>,
  claims: BasicClaims,
  tenent: TenentRow,
  code: String,
) -> impl IntoResponse {
  if claims.kind != TOKEN_TYPE_MFA_TOTP {
    return Errors::unauthorized()
      .with_error(AUTHORIZATION_HEADER, "invalid-token-type")
      .into_response();
  }
  let user = match get_user_by_id(pool, claims.sub).await {
    Ok(Some(user)) => user,
    Ok(None) => {
      return Errors::not_found()
        .with_error("user", REQUIRED_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("Error getting user: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  let totp = match get_user_totp_by_user_id(pool, user.id).await {
    Ok(Some(totp)) => totp,
    Ok(None) => {
      return Errors::not_found()
        .with_error("totp", REQUIRED_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("Error getting user TOTP: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  match totp.verify(&code) {
    Ok(true) => (),
    Ok(false) => {
      return Errors::unauthorized()
        .with_error("totp", INVALID_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("Error verifying TOTP: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  create_user_token(
    pool,
    tenent,
    user,
    Some(claims.scopes.join(" ")),
    TOKEN_ISSUED_TYPE_MFA.to_owned(),
    true,
  )
  .await
  .into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new().route("/mfa", post(mfa)).with_state(state)
}
