use std::collections::HashMap;

use crate::{
  core::error::{Errors, INTERNAL_ERROR, NOT_FOUND_ERROR},
  middleware::{json::Json, service_account_authorization::ServiceAccountAuthorization},
  model::{
    tenent::{CreateTenent, Tenent, TenentQuery, UpdateTenent},
    tenent_oauth2_provider::TenentOAuth2Provider,
    util::{OffsetAndLimit, Pagination, DEFAULT_LIMIT},
  },
  repository::{
    self,
    tenent::get_tenents,
    tenent_oauth2_provider::{
      get_tenent_oauth2_providers, get_tenents_oauth2_providers, TenentOAuth2ProviderRow,
    },
  },
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
    all_tenents,
    get_tenent_by_id,
    get_tenent_by_client_id,
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
    TenentQuery
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
pub async fn all_tenents(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Query(offset_and_limit): Query<OffsetAndLimit>,
  Query(query): Query<TenentQuery>,
) -> impl IntoResponse {
  let limit = offset_and_limit.limit.unwrap_or(DEFAULT_LIMIT);
  let offset = offset_and_limit.offset.unwrap_or(0);
  let (rows, oauth2_providers) = match tokio::try_join!(
    get_tenents(&state.pool, limit, offset),
    get_tenents_oauth2_providers(&state.pool, limit, offset),
  ) {
    Ok(results) => results,
    Err(e) => {
      log::error!("error getting tenents: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let mut oauth2_providers_by_id: HashMap<i64, Vec<TenentOAuth2ProviderRow>> = oauth2_providers
    .into_iter()
    .fold(HashMap::new(), |mut acc, row| {
      acc.entry(row.tenent_id).or_default().push(row);
      acc
    });
  let show_private_key = query.show_private_key.unwrap_or(false);
  let tenents = rows
    .into_iter()
    .map(|row| {
      let private_key = row.private_key.clone();
      let mut tenent = Tenent::from(row);
      for oauth2_provider in oauth2_providers_by_id
        .remove(&tenent.id)
        .unwrap_or_default()
      {
        tenent
          .oauth2_providers
          .push(TenentOAuth2Provider::from(oauth2_provider));
      }
      if show_private_key {
        tenent.private_key = Some(private_key);
      }
      tenent
    })
    .collect::<Vec<_>>();

  axum::Json(Pagination {
    has_more: tenents.len() == limit,
    items: tenents,
  })
  .into_response()
}

#[utoipa::path(
  get,
  path = "tenents/by-client-id/{tenent_client_id}",
  tags = ["tenent"],
  params(
    ("tenent_client_id" = uuid::Uuid, Path, description = "Tenent ID"),
    TenentQuery,
  ),
  responses(
    (status = 200, content_type = "application/json", body = Tenent),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn get_tenent_by_client_id(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(tenent_client_id): Path<uuid::Uuid>,
  Query(query): Query<TenentQuery>,
) -> impl IntoResponse {
  let row_optional =
    match repository::tenent::get_tenent_by_client_id(&state.pool, &tenent_client_id.to_string())
      .await
    {
      Ok(row) => row,
      Err(e) => {
        log::error!("error getting tenent: {}", e);
        return Errors::internal_error()
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    };
  let row = match row_optional {
    Some(row) => row,
    None => {
      return Errors::not_found()
        .with_error("tenent", NOT_FOUND_ERROR)
        .into_response()
    }
  };

  let oauth2_providers = match get_tenent_oauth2_providers(&state.pool, row.id).await {
    Ok(results) => results,
    Err(e) => {
      log::error!("error getting tenents: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let private_key = row.private_key.clone();
  let mut tenent = Tenent::from(row);
  for oauth2_provider in oauth2_providers {
    tenent.oauth2_providers.push(oauth2_provider.into());
  }
  if query.show_private_key.unwrap_or(false) {
    tenent.private_key = Some(private_key);
  }
  axum::Json(tenent).into_response()
}

#[utoipa::path(
  get,
  path = "tenents/{tenent_id}",
  tags = ["tenent"],
  params(
    ("tenent_id" = i64, Path, description = "Tenent ID"),
    TenentQuery,
  ),
  responses(
    (status = 200, content_type = "application/json", body = Tenent),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn get_tenent_by_id(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(tenent_id): Path<i64>,
  Query(query): Query<TenentQuery>,
) -> impl IntoResponse {
  let (row_optional, oauth2_providers) = match tokio::try_join!(
    repository::tenent::get_tenent_by_id(&state.pool, tenent_id),
    get_tenent_oauth2_providers(&state.pool, tenent_id),
  ) {
    Ok(results) => results,
    Err(e) => {
      log::error!("error getting tenents: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let row = match row_optional {
    Some(row) => row,
    None => {
      return Errors::not_found()
        .with_error("tenent", NOT_FOUND_ERROR)
        .into_response()
    }
  };
  let private_key = row.private_key.clone();
  let mut tenent = Tenent::from(row);
  for oauth2_provider in oauth2_providers {
    tenent.oauth2_providers.push(oauth2_provider.into());
  }
  if query.show_private_key.unwrap_or(false) {
    tenent.private_key = Some(private_key);
  }
  axum::Json(tenent).into_response()
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
  let algorithm = payload.algorithm.unwrap_or_default();
  let (public_key, private_key) = algorithm.keys(payload.public_key, payload.private_key);
  let tenent_row = match repository::tenent::create_tenent(
    &state.pool,
    repository::tenent::CreateTenent {
      client_id: payload
        .client_id
        .unwrap_or_else(uuid::Uuid::new_v4)
        .to_string(),
      issuer: payload.issuer,
      audience: payload.audience,
      algorithm: algorithm.to_string(),
      public_key: public_key,
      private_key: private_key,
      expires_in_seconds: payload.expires_in_seconds.unwrap_or(86400),
      refresh_expires_in_seconds: payload.refresh_expires_in_seconds.unwrap_or(604800),
    },
  )
  .await
  {
    Ok(tenent_row) => tenent_row,
    Err(e) => {
      log::error!("error creating tenent: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let private_key = tenent_row.private_key.clone();
  let mut tenent = Tenent::from(tenent_row);
  tenent.private_key = Some(private_key);
  axum::Json(tenent).into_response()
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
    Ok(None) => {
      return Errors::not_found()
        .with_error("tenent", NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error creating tenent: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  axum::Json(Tenent::from(tenent)).into_response()
}

#[utoipa::path(
  delete,
  path = "tenents/{tenent_id}",
  tags = ["tenent"],
  params(
    ("tenent_id" = i64, Path, description = "Tenent ID")
  ),
  responses(
    (status = 204),
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
    Ok(None) => {
      return Errors::not_found()
        .with_error("tenent", NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error creating tenent: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }
  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route("/tenents", get(all_tenents))
    .route(
      "/tenents/by-client-id/{tenent_client_id}",
      get(get_tenent_by_client_id),
    )
    .route("/tenents/{tenent_id}", get(get_tenent_by_id))
    .route("/tenents", post(create_tenent))
    .route("/tenents/{tenent_id}", put(update_tenent))
    .route("/tenents/{tenent_id}", delete(delete_tenent))
    .with_state(state)
}
