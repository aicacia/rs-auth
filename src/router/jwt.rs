use std::str::FromStr;

use axum::{extract::State, response::IntoResponse};
use http::{HeaderMap, StatusCode};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  core::{
    error::{
      Errors, InternalError, INTERNAL_ERROR, INVALID_ERROR, NOT_ALLOWED_ERROR, NOT_FOUND_ERROR,
      REQUIRED_ERROR,
    },
    openapi::{AUTHORIZATION_HEADER, TENENT_ID_HEADER},
  },
  middleware::{
    authorization::parse_authorization,
    claims::{tenant_encoding_key, TOKEN_TYPE_BEARER},
    json::Json,
    service_account_authorization::ServiceAccountAuthorization,
  },
  model::jwt::JWTRequest,
  repository::tenant::get_tenant_by_id,
};

use super::RouterState;

pub const JWT_TAG: &str = "jwt";

#[utoipa::path(
  post,
  path = "/jwt",
  tags = [JWT_TAG],
  request_body = JWTRequest,
  responses(
    (status = 201, content_type = "text/plain", body = String),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn create_jwt(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Json(payload): Json<JWTRequest>,
) -> impl IntoResponse {
  let tenant = match get_tenant_by_id(&state.pool, payload.tenant_id).await {
    Ok(Some(tenant)) => tenant,
    Ok(None) => {
      return InternalError::bad_request()
        .with_error("tenant_id", NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("failed to get tenant by id: {e}");
      return InternalError::internal_error()
        .with_error("tenant_id", INTERNAL_ERROR)
        .into_response();
    }
  };

  let algorithm = match jsonwebtoken::Algorithm::from_str(&tenant.algorithm) {
    Ok(algorithm) => algorithm,
    Err(_) => {
      return InternalError::bad_request()
        .with_error("algorithm", NOT_ALLOWED_ERROR)
        .into_response()
    }
  };

  let mut header = jsonwebtoken::Header::new(algorithm);
  header.kid = Some(tenant.id.to_string());

  let key = match tenant_encoding_key(&tenant, algorithm) {
    Ok(key) => key,
    Err(_) => {
      return InternalError::bad_request()
        .with_error("algorithm", NOT_ALLOWED_ERROR)
        .into_response()
    }
  };
  let token = match jsonwebtoken::encode(&header, &payload.claims, &key) {
    Ok(token) => token,
    Err(_) => {
      return InternalError::internal_error()
        .with_error("jwt", INTERNAL_ERROR)
        .into_response()
    }
  };

  (StatusCode::CREATED, token).into_response()
}

#[utoipa::path(
  get,
  path = "/jwt",
  tags = [JWT_TAG],
  params(
    ("Tenant-ID" = String, Header, description = "Tenant UUID"),
  ),
  responses(
    (status = 200, content_type = "application/json", body = serde_json::Map<String, serde_json::Value>),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = []),
  )
)]
pub async fn jwt_is_valid(
  State(state): State<RouterState>,
  headers: HeaderMap,
) -> impl IntoResponse {
  let authorization_string = match headers.get(AUTHORIZATION_HEADER) {
    Some(authorization_header_value) => match authorization_header_value.to_str() {
      Ok(authorization_string) => {
        if authorization_string.len() < TOKEN_TYPE_BEARER.len() + 1 {
          log::error!("invalid authorization header is invalid");
          return InternalError::unauthorized()
            .with_error(AUTHORIZATION_HEADER, INVALID_ERROR)
            .into_response();
        }
        &authorization_string[(TOKEN_TYPE_BEARER.len() + 1)..]
      }
      Err(e) => {
        log::error!("invalid authorization header is invalid: {}", e);
        return InternalError::unauthorized()
          .with_error(AUTHORIZATION_HEADER, INVALID_ERROR)
          .into_response();
      }
    },
    None => {
      log::error!("invalid authorization header is missing");
      return InternalError::unauthorized()
        .with_error(AUTHORIZATION_HEADER, REQUIRED_ERROR)
        .into_response();
    }
  };
  let tenant_client_id = match headers.get(TENENT_ID_HEADER) {
    Some(tenant_client_id_value) => match tenant_client_id_value.to_str() {
      Ok(tenant_client_id_string) => match uuid::Uuid::from_str(tenant_client_id_string) {
        Ok(client_id) => client_id,
        Err(e) => {
          log::error!("invalid tenant header is invalid: {}", e);
          return InternalError::unauthorized()
            .with_error(TENENT_ID_HEADER, INVALID_ERROR)
            .into_response();
        }
      },
      Err(e) => {
        log::error!("invalid tenant header is invalid: {}", e);
        return InternalError::unauthorized()
          .with_error(TENENT_ID_HEADER, INVALID_ERROR)
          .into_response();
      }
    },
    None => {
      log::error!("invalid tenant header is missing");
      return InternalError::unauthorized()
        .with_error(TENENT_ID_HEADER, REQUIRED_ERROR)
        .into_response();
    }
  };

  let (tenant, token) = match parse_authorization::<serde_json::Map<String, serde_json::Value>>(
    &state.pool,
    authorization_string,
  )
  .await
  {
    Ok(result) => result,
    Err(e) => {
      log::error!("Error parsing authorization: {}", e);
      return InternalError::unauthorized()
        .with_error(AUTHORIZATION_HEADER, INVALID_ERROR)
        .into_response();
    }
  };
  let token_tenant_client_id = match uuid::Uuid::from_str(&tenant.client_id) {
    Ok(client_id) => client_id,
    Err(e) => {
      log::error!("authorization tenant id is not a valid uuid: {}", e);
      return InternalError::unauthorized()
        .with_error("tenant", INVALID_ERROR)
        .into_response();
    }
  };
  if tenant_client_id != token_tenant_client_id {
    return InternalError::unauthorized()
      .with_error("tenant", INVALID_ERROR)
      .into_response();
  }
  axum::Json(token.claims).into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(jwt_is_valid, create_jwt))
    .with_state(state)
}
