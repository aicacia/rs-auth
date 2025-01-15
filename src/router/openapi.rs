use axum::extract::State;
use utoipa::openapi::{OpenApi as OpenApiSpec, RefOr, Schema};
use utoipa_axum::{router::OpenApiRouter, routes};

pub const OPENAPI_TAG: &str = "openapi";

#[utoipa::path(
  get,
  path = "/openapi.json",
  tags = [OPENAPI_TAG],
  responses(
    (status = 200, description = "OpenApi documenation"),
  )
)]
pub async fn get_openapi(State(openapi): State<OpenApiSpec>) -> axum::Json<OpenApiSpec> {
  axum::Json(openapi)
}

pub fn create_router(mut openapi_spec: OpenApiSpec) -> OpenApiRouter {
  let mut schemas = Vec::<(String, RefOr<Schema>)>::new();
  let (path, item, types) = routes!(@resolve_types get_openapi : schemas);
  openapi_spec.paths.add_path_operation(path, types, item);

  OpenApiRouter::new()
    .routes(routes!(get_openapi))
    .with_state(openapi_spec)
}
