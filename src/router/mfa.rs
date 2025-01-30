use axum::{extract::State, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  core::{
    error::{Errors, InternalError, INTERNAL_ERROR, INVALID_ERROR, NOT_FOUND_ERROR},
    openapi::AUTHORIZATION_HEADER,
  },
  middleware::{
    authorization::{parse_authorization, Authorization},
    claims::{
      BasicClaims, TOKEN_SUB_TYPE_SERVICE_ACCOUNT, TOKEN_TYPE_BEARER, TOKEN_TYPE_MFA_TOTP_PREFIX,
    },
    json::Json,
  },
  model::{
    mfa::MFARequest,
    token::{Token, TOKEN_ISSUED_TYPE_MFA},
  },
  repository::{
    tenant::TenantRow,
    user::{get_user_by_id, UserRow},
    user_totp::get_user_totp_by_user_id,
  },
};

use super::{token::create_user_token, RouterState};

pub const MFA_TAG: &str = "mfa";

#[utoipa::path(
  post,
  path = "/mfa",
  tags = [MFA_TAG],
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
  Authorization { claims, tenant, .. }: Authorization,
  Json(payload): Json<MFARequest>,
) -> impl IntoResponse {
  if !claims.r#type.starts_with(TOKEN_TYPE_MFA_TOTP_PREFIX) {
    return InternalError::unauthorized()
      .with_error(AUTHORIZATION_HEADER, "invalid-token-type")
      .into_response();
  }
  let mfa_type = &claims.r#type[(TOKEN_TYPE_MFA_TOTP_PREFIX.len())..];
  log::debug!("MFA type: {}", mfa_type);
  let user = match get_user_by_id(&state.pool, claims.sub).await {
    Ok(Some(user)) => user,
    Ok(None) => {
      return InternalError::not_found()
        .with_error("user", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("Error getting user: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  match payload {
    MFARequest::TOTP { code } => totp_request(&state.pool, user, claims, tenant, code)
      .await
      .into_response(),
    MFARequest::ServiceAccount { code } => {
      service_account_request(&state.pool, user, claims, tenant, code)
        .await
        .into_response()
    }
  }
}

async fn totp_request(
  pool: &sqlx::AnyPool,
  user: UserRow,
  claims: BasicClaims,
  tenant: TenantRow,
  code: String,
) -> impl IntoResponse {
  let totp = match get_user_totp_by_user_id(pool, user.id).await {
    Ok(Some(totp)) => totp,
    Ok(None) => {
      return InternalError::not_found()
        .with_error("totp", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("Error getting user TOTP: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  match totp.verify(&code) {
    Ok(true) => (),
    Ok(false) => {
      return InternalError::unauthorized()
        .with_error("totp", INVALID_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("Error verifying TOTP: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }

  create_user_token(
    pool,
    tenant,
    user,
    Some(claims.scopes.join(" ")),
    Some(TOKEN_ISSUED_TYPE_MFA.to_owned()),
    true,
  )
  .await
  .into_response()
}

async fn service_account_request(
  pool: &sqlx::AnyPool,
  user: UserRow,
  claims: BasicClaims,
  tenant: TenantRow,
  service_account_token: String,
) -> impl IntoResponse {
  let service_account_claims =
    match parse_authorization::<BasicClaims>(pool, &service_account_token).await {
      Ok((_, claims)) => claims,
      Err(e) => {
        return e.into_response();
      }
    };
  if service_account_claims.claims.r#type != TOKEN_TYPE_BEARER
    || service_account_claims.claims.sub_type != TOKEN_SUB_TYPE_SERVICE_ACCOUNT
  {
    return InternalError::unauthorized()
      .with_error("token", "invalid-token-sub-type")
      .into_response();
  }
  create_user_token(
    pool,
    tenant,
    user,
    Some(claims.scopes.join(" ")),
    Some(TOKEN_ISSUED_TYPE_MFA.to_owned()),
    true,
  )
  .await
  .into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new().routes(routes!(mfa)).with_state(state)
}
