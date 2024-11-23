use axum::{extract::State, routing::get, Json, Router};
use utoipa::{openapi::OpenApi as OpenApiSpec, OpenApi};

#[derive(OpenApi)]
#[openapi(
  paths(
    openapi,
  ),
  tags(
    (name = "openapi", description = "OpenApi endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  get,
  path = "openapi.json",
  responses(
    (status = 200, description = "OpenApi documenation"),
  )
)]
pub async fn openapi(State(openapi): State<OpenApiSpec>) -> Json<OpenApiSpec> {
  Json(openapi)
}

pub fn create_router(openapi_spec: OpenApiSpec) -> Router {
  Router::new()
    .route("/openapi.json", get(openapi))
    .with_state(openapi_spec)
}
