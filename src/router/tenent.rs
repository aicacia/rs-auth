use crate::{
  core::error::Errors,
  middleware::{json::Json, service_account_authorization::ServiceAccountAuthorization},
  model::{
    tenent::{CreateTenent, Tenent, UpdateTenent},
    util::{OffsetAndLimit, Pagination, DEFAULT_LIMIT},
  },
  repository::{self, tenent::get_tenents},
};

use axum::{
  extract::{Path, Query, State},
  response::IntoResponse,
  routing::{delete, get, post, put},
  Router,
};
use http::StatusCode;
use utoipa::OpenApi;

use super::RouterState;

#[derive(OpenApi)]
#[openapi(
  paths(
    tenents,
    create_tenent,
    update_tenent,
    delete_tenent
  ),
  tags(
    (name = "tenent", description = "Tenent endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  get,
  path = "tenents",
  tags = ["tenent"],
  params(
    OffsetAndLimit,
  ),
  responses(
    (status = 200, content_type = "application/json", body = Pagination<Tenent>),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn tenents(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Query(query): Query<OffsetAndLimit>,
) -> impl IntoResponse {
  let limit = query.limit.unwrap_or(DEFAULT_LIMIT);
  let offset = query.offset.unwrap_or(0);
  let rows = match get_tenents(&state.pool, limit, offset).await {
    Ok(results) => results,
    Err(e) => {
      log::error!("error getting tenents: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }
  };
  let tenents = rows.into_iter().map(Into::into).collect::<Vec<Tenent>>();

  axum::Json(Pagination {
    has_more: tenents.len() == limit,
    items: tenents,
  })
  .into_response()
}

#[utoipa::path(
  post,
  path = "tenents",
  tags = ["tenent"],
  request_body = CreateTenent,
  responses(
    (status = 201, content_type = "application/json", body = Tenent),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn create_tenent(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Json(payload): Json<CreateTenent>,
) -> impl IntoResponse {
  let tenent = match repository::tenent::create_tenent(
    &state.pool,
    repository::tenent::CreateTenent {
      client_id: payload
        .client_id
        .unwrap_or_else(uuid::Uuid::new_v4)
        .to_string(),
      issuer: payload.issuer,
      audience: payload.audience,
      algorithm: payload.algorithm.unwrap_or_default().to_string(),
      public_key: payload.public_key,
      private_key: payload.private_key,
      expires_in_seconds: payload.expires_in_seconds.unwrap_or(86400),
      refresh_expires_in_seconds: payload.refresh_expires_in_seconds.unwrap_or(604800),
    },
  )
  .await
  {
    Ok(tenent) => tenent,
    Err(e) => {
      log::error!("error creating tenent: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }
  };
  axum::Json(Into::<Tenent>::into(tenent)).into_response()
}

#[utoipa::path(
  put,
  path = "tenents/{tenent_id}",
  tags = ["tenent"],
  request_body = UpdateTenent,
  params(
    ("tenent_id" = i64, Path, description = "Tenent ID")
  ),
  responses(
    (status = 201, content_type = "application/json", body = Tenent),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn update_tenent(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(tenent_id): Path<i64>,
  Json(payload): Json<UpdateTenent>,
) -> impl IntoResponse {
  let tenent = match repository::tenent::update_tenent(
    &state.pool,
    tenent_id,
    repository::tenent::UpdateTenent {
      client_id: payload.client_id.as_ref().map(ToString::to_string),
      issuer: payload.issuer,
      audience: payload.audience,
      algorithm: payload.algorithm.as_ref().map(ToString::to_string),
      public_key: payload.public_key,
      private_key: payload.private_key,
      expires_in_seconds: payload.expires_in_seconds,
      refresh_expires_in_seconds: payload.refresh_expires_in_seconds,
    },
  )
  .await
  {
    Ok(Some(tenent)) => tenent,
    Ok(None) => return Errors::from(StatusCode::NOT_FOUND).into_response(),
    Err(e) => {
      log::error!("error creating tenent: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }
  };
  axum::Json(Into::<Tenent>::into(tenent)).into_response()
}

#[utoipa::path(
  delete,
  path = "tenents/{tenent_id}",
  tags = ["tenent"],
  params(
    ("tenent_id" = i64, Path, description = "Tenent ID")
  ),
  responses(
    (status = 201, content_type = "application/json", body = Tenent),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn delete_tenent(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(tenent_id): Path<i64>,
) -> impl IntoResponse {
  match repository::tenent::delete_tenent(&state.pool, tenent_id).await {
    Ok(Some(_)) => {}
    Ok(None) => return Errors::from(StatusCode::NOT_FOUND).into_response(),
    Err(e) => {
      log::error!("error creating tenent: {}", e);
      return Errors::from(StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }
  }
  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/tenents", get(tenents))
    .route("/tenents", post(create_tenent))
    .route("/tenents/{tenent_id}", put(update_tenent))
    .route("/tenents/{tenent_id}", delete(update_tenent))
    .with_state(state)
}
