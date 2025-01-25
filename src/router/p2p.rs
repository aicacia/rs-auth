use crate::{
  core::{error::Errors, openapi::AUTHORIZATION_HEADER},
  middleware::{
    authorization::Authorization,
    claims::{TOKEN_SUB_TYPE_SERVICE_ACCOUNT, TOKEN_SUB_TYPE_USER, TOKEN_TYPE_BEARER},
  },
  model::p2p::P2P,
};

use axum::{extract::State, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

use super::RouterState;

pub const P2P_TAG: &str = "p2p";

#[utoipa::path(
  get,
  path = "/p2p",
  tags = [P2P_TAG],
  responses(
    (status = 200, description = "P2P response", body = P2P),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn p2p(
  State(state): State<RouterState>,
  Authorization { claims, .. }: Authorization,
) -> impl IntoResponse {
  if claims.kind != TOKEN_TYPE_BEARER
    || (claims.sub_kind != TOKEN_SUB_TYPE_USER && claims.sub_kind != TOKEN_SUB_TYPE_SERVICE_ACCOUNT)
  {
    return Errors::unauthorized()
      .with_error(AUTHORIZATION_HEADER, "invalid-token-type")
      .into_response();
  }
  axum::Json(P2P::new(state.config.as_ref())).into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new().routes(routes!(p2p)).with_state(state)
}
