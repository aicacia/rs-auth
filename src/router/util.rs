use crate::model::util::{Health, Version};

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use utoipa::OpenApi;

use super::RouterState;

#[derive(OpenApi)]
#[openapi(
  paths(
    health,
    version,
  ),
  components(
    schemas(
      Version,
      Health,
    )
  ),
  tags(
    (name = "util", description = "Utility endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  get,
  path = "health",
  tags = ["util"],
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

  (status, Json(health))
}

#[utoipa::path(
  get,
  path = "version",
  tags = ["util"],
  responses(
    (status = 200, description = "Version response", body = Version),
  )
)]
pub async fn version() -> Json<Version> {
  Json(Version::default())
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/health", get(health))
    .route("/version", get(version))
    .with_state(state)
}
