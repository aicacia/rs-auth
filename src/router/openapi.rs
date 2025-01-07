use axum::extract::State;
use utoipa::openapi::{
  path::OperationBuilder, HttpMethod, OpenApi as OpenApiSpec, ResponseBuilder,
};
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
  // TODO: use the description from get_openapi
  openapi_spec.paths.add_path_operation(
    "/openapi.json",
    vec![HttpMethod::Get],
    OperationBuilder::new()
      .tag(OPENAPI_TAG)
      .response(
        "200",
        ResponseBuilder::new()
          .description("OpenApi documenation")
          .build(),
      )
      .build(),
  );

  OpenApiRouter::new()
    .routes(routes!(get_openapi))
    .with_state(openapi_spec)
}
