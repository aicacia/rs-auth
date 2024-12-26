use std::str::FromStr;

use axum::{
  extract::State,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use http::{HeaderMap, StatusCode};
use utoipa::OpenApi;

use crate::{
  core::{
    error::{
      Errors, INTERNAL_ERROR, INVALID_ERROR, NOT_ALLOWED_ERROR, NOT_FOUND_ERROR, REQUIRED_ERROR,
    },
    openapi::AUTHORIZATION_HEADER,
  },
  middleware::{
    authorization::parse_authorization,
    claims::{tenent_encoding_key, TOKEN_TYPE_BEARER},
    json::Json,
    service_account_authorization::ServiceAccountAuthorization,
  },
  model::jwt::JWTRequest,
  repository::tenent::get_tenent_by_id,
};

use super::RouterState;

#[derive(OpenApi)]
#[openapi(paths(create_jwt, jwt_is_valid))]
pub struct ApiDoc;

#[utoipa::path(
  post,
  path = "jwt",
  tags = ["jwt"],
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
  let tenent = match get_tenent_by_id(&state.pool, payload.tenent_id).await {
    Ok(Some(tenent)) => tenent,
    Ok(None) => {
      return Errors::bad_request()
        .with_error("tenent_id", NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("failed to get tenent by id: {e}");
      return Errors::internal_error()
        .with_error("tenent_id", INTERNAL_ERROR)
        .into_response();
    }
  };

  let algorithm = match jsonwebtoken::Algorithm::from_str(&tenent.algorithm) {
    Ok(algorithm) => algorithm,
    Err(_) => {
      return Errors::bad_request()
        .with_error("algorithm", NOT_ALLOWED_ERROR)
        .into_response()
    }
  };

  let mut header = jsonwebtoken::Header::new(algorithm);
  header.kid = Some(tenent.id.to_string());

  let key = match tenent_encoding_key(&tenent, algorithm) {
    Ok(key) => key,
    Err(_) => {
      return Errors::bad_request()
        .with_error("algorithm", NOT_ALLOWED_ERROR)
        .into_response()
    }
  };
  let token = match jsonwebtoken::encode(&header, &payload.claims, &key) {
    Ok(token) => token,
    Err(_) => {
      return Errors::internal_error()
        .with_error("jwt", INTERNAL_ERROR)
        .into_response()
    }
  };

  (StatusCode::CREATED, token).into_response()
}

#[utoipa::path(
  get,
  path = "jwt",
  tags = ["jwt"],
  responses(
    (status = 204),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
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
          log::error!("invalid authorization header is missing");
          return Errors::unauthorized()
            .with_error(AUTHORIZATION_HEADER, REQUIRED_ERROR)
            .into_response();
        }
        &authorization_string[(TOKEN_TYPE_BEARER.len() + 1)..]
      }
      Err(e) => {
        log::error!("invalid authorization header is missing: {}", e);
        return Errors::unauthorized()
          .with_error(AUTHORIZATION_HEADER, REQUIRED_ERROR)
          .into_response();
      }
    },
    None => "",
  };

  match parse_authorization::<serde_json::Map<String, serde_json::Value>>(
    &state.pool,
    authorization_string,
  )
  .await
  {
    Ok(_) => (StatusCode::NO_CONTENT, ()).into_response(),
    Err(e) => {
      log::error!("Error parsing authorization: {}", e);
      Errors::unauthorized()
        .with_error(AUTHORIZATION_HEADER, INVALID_ERROR)
        .into_response()
    }
  }
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/jwt", get(jwt_is_valid))
    .route("/jwt", post(create_jwt))
    .with_state(state)
}
