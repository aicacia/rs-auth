use axum::{extract::State, routing::get, Router};
use utoipa::{openapi::OpenApi as OpenApiSpec, OpenApi};

#[derive(OpenApi)]
#[openapi(
  paths(
    get_openapi,
  ),
  tags(
    (name = "openapi", description = "OpenApi endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  get,
  path = "openapi.json",
  tags = ["openapi"],
  responses(
    (status = 200, description = "OpenApi documenation"),
  )
)]
pub async fn get_openapi(State(openapi): State<OpenApiSpec>) -> axum::Json<OpenApiSpec> {
  axum::Json(openapi)
}

pub fn create_router(openapi_spec: OpenApiSpec) -> Router {
  Router::new()
    .route("/openapi.json", get(get_openapi))
    .with_state(openapi_spec)
}
