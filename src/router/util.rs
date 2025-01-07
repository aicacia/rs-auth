use crate::model::util::{Health, Version};

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

use super::RouterState;

pub const UTIL_TAG: &str = "util";

#[utoipa::path(
  get,
  path = "/health",
  tags = [UTIL_TAG],
  responses(
    (status = 200, description = "Health check response", body = Health),
    (status = 500, description = "Health check response", body = Health),
  )
)]
pub async fn health(State(state): State<RouterState>) -> impl IntoResponse {
  let health = Health {
    db: !state.pool.is_closed(),
  };

  let status = if health.is_healthy() {
    StatusCode::OK
  } else {
    StatusCode::INTERNAL_SERVER_ERROR
  };

  (status, axum::Json(health))
}

#[utoipa::path(
  get,
  path = "/version",
  tags = [UTIL_TAG],
  responses(
    (status = 200, description = "Version response", body = Version),
  )
)]
pub async fn version() -> axum::Json<Version> {
  axum::Json(Version::default())
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(health))
    .routes(routes!(version))
    .with_state(state)
}
