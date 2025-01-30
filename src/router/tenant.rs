use std::collections::HashMap;

use crate::{
  core::error::{Errors, InternalError, INTERNAL_ERROR, NOT_FOUND_ERROR},
  middleware::{json::Json, service_account_authorization::ServiceAccountAuthorization},
  model::{
    tenant::{CreateTenant, Tenant, TenantPagination, TenantQuery, UpdateTenant},
    tenant_oauth2_provider::TenantOAuth2Provider,
    util::{OffsetAndLimit, Pagination, DEFAULT_LIMIT},
  },
  repository::{
    self,
    tenant::get_tenants,
    tenant_oauth2_provider::{
      get_tenant_oauth2_providers, get_tenants_oauth2_providers, TenantOAuth2ProviderRow,
    },
  },
};

use axum::{
  extract::{Path, Query, State},
  response::IntoResponse,
};
use http::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::RouterState;

pub const TENANT_TAG: &str = "tenant";

#[utoipa::path(
  get,
  path = "/tenants",
  tags = [TENANT_TAG],
  params(
    OffsetAndLimit,
    TenantQuery
  ),
  responses(
    (status = 200, content_type = "application/json", body = TenantPagination),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn all_tenants(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Query(offset_and_limit): Query<OffsetAndLimit>,
  Query(query): Query<TenantQuery>,
) -> impl IntoResponse {
  let limit = offset_and_limit.limit.unwrap_or(DEFAULT_LIMIT);
  let offset = offset_and_limit.offset.unwrap_or(0);
  let (rows, oauth2_providers) = match tokio::try_join!(
    get_tenants(&state.pool, limit, offset),
    get_tenants_oauth2_providers(&state.pool, limit, offset),
  ) {
    Ok(results) => results,
    Err(e) => {
      log::error!("error getting tenants: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let mut oauth2_providers_by_id: HashMap<i64, Vec<TenantOAuth2ProviderRow>> = oauth2_providers
    .into_iter()
    .fold(HashMap::new(), |mut acc, row| {
      acc.entry(row.tenant_id).or_default().push(row);
      acc
    });
  let show_private_key = query.show_private_key.unwrap_or(false);
  let tenants = rows
    .into_iter()
    .map(|row| {
      let private_key = row.private_key.clone();
      let mut tenant = Tenant::from(row);
      for oauth2_provider in oauth2_providers_by_id
        .remove(&tenant.id)
        .unwrap_or_default()
      {
        tenant.oauth2_providers.push(TenantOAuth2Provider::from((
          state.config.as_ref(),
          oauth2_provider,
        )));
      }
      if show_private_key {
        tenant.private_key = Some(private_key);
      }
      tenant
    })
    .collect::<Vec<_>>();

  axum::Json(Pagination {
    has_more: tenants.len() == limit,
    items: tenants,
  })
  .into_response()
}

#[utoipa::path(
  get,
  path = "/tenants/by-client-id/{tenant_client_id}",
  tags = [TENANT_TAG],
  params(
    ("tenant_client_id" = uuid::Uuid, Path, description = "Tenant ID"),
    TenantQuery,
  ),
  responses(
    (status = 200, content_type = "application/json", body = Tenant),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn get_tenant_by_client_id(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(tenant_client_id): Path<uuid::Uuid>,
  Query(query): Query<TenantQuery>,
) -> impl IntoResponse {
  let row_optional =
    match repository::tenant::get_tenant_by_client_id(&state.pool, &tenant_client_id.to_string())
      .await
    {
      Ok(row) => row,
      Err(e) => {
        log::error!("error getting tenant: {}", e);
        return InternalError::internal_error()
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    };
  let row = match row_optional {
    Some(row) => row,
    None => {
      return InternalError::not_found()
        .with_error(TENANT_TAG, NOT_FOUND_ERROR)
        .into_response()
    }
  };

  let oauth2_providers = match get_tenant_oauth2_providers(&state.pool, row.id).await {
    Ok(results) => results,
    Err(e) => {
      log::error!("error getting tenants: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let private_key = row.private_key.clone();
  let mut tenant = Tenant::from(row);
  for oauth2_provider in oauth2_providers {
    tenant
      .oauth2_providers
      .push((state.config.as_ref(), oauth2_provider).into());
  }
  if query.show_private_key.unwrap_or(false) {
    tenant.private_key = Some(private_key);
  }
  axum::Json(tenant).into_response()
}

#[utoipa::path(
  get,
  path = "/tenants/{tenant_id}",
  tags = [TENANT_TAG],
  params(
    ("tenant_id" = i64, Path, description = "Tenant ID"),
    TenantQuery,
  ),
  responses(
    (status = 200, content_type = "application/json", body = Tenant),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn get_tenant_by_id(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(tenant_id): Path<i64>,
  Query(query): Query<TenantQuery>,
) -> impl IntoResponse {
  let (row_optional, oauth2_providers) = match tokio::try_join!(
    repository::tenant::get_tenant_by_id(&state.pool, tenant_id),
    get_tenant_oauth2_providers(&state.pool, tenant_id),
  ) {
    Ok(results) => results,
    Err(e) => {
      log::error!("error getting tenants: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let row = match row_optional {
    Some(row) => row,
    None => {
      return InternalError::not_found()
        .with_error(TENANT_TAG, NOT_FOUND_ERROR)
        .into_response()
    }
  };
  let private_key = row.private_key.clone();
  let mut tenant = Tenant::from(row);
  for oauth2_provider in oauth2_providers {
    tenant
      .oauth2_providers
      .push((state.config.as_ref(), oauth2_provider).into());
  }
  if query.show_private_key.unwrap_or(false) {
    tenant.private_key = Some(private_key);
  }
  axum::Json(tenant).into_response()
}

#[utoipa::path(
  post,
  path = "/tenants",
  tags = [TENANT_TAG],
  request_body = CreateTenant,
  responses(
    (status = 201, content_type = "application/json", body = Tenant),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn create_tenant(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Json(payload): Json<CreateTenant>,
) -> impl IntoResponse {
  let algorithm = payload.algorithm.unwrap_or_default();
  let (public_key, private_key) = algorithm.keys(payload.public_key, payload.private_key);
  let tenant_row = match repository::tenant::create_tenant(
    &state.pool,
    repository::tenant::CreateTenant {
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
    Ok(tenant_row) => tenant_row,
    Err(e) => {
      log::error!("error creating tenant: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let private_key = tenant_row.private_key.clone();
  let mut tenant = Tenant::from(tenant_row);
  tenant.private_key = Some(private_key);
  axum::Json(tenant).into_response()
}

#[utoipa::path(
  put,
  path = "/tenants/{tenant_id}",
  tags = [TENANT_TAG],
  request_body = UpdateTenant,
  params(
    ("tenant_id" = i64, Path, description = "Tenant ID")
  ),
  responses(
    (status = 201, content_type = "application/json", body = Tenant),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn update_tenant(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(tenant_id): Path<i64>,
  Json(payload): Json<UpdateTenant>,
) -> impl IntoResponse {
  let tenant = match repository::tenant::update_tenant(
    &state.pool,
    tenant_id,
    repository::tenant::UpdateTenant {
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
    Ok(Some(tenant)) => tenant,
    Ok(None) => {
      return InternalError::not_found()
        .with_error(TENANT_TAG, NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error creating tenant: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  axum::Json(Tenant::from(tenant)).into_response()
}

#[utoipa::path(
  delete,
  path = "/tenants/{tenant_id}",
  tags = [TENANT_TAG],
  params(
    ("tenant_id" = i64, Path, description = "Tenant ID")
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
pub async fn delete_tenant(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(tenant_id): Path<i64>,
) -> impl IntoResponse {
  match repository::tenant::delete_tenant(&state.pool, tenant_id).await {
    Ok(Some(_)) => {}
    Ok(None) => {
      return InternalError::not_found()
        .with_error(TENANT_TAG, NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error creating tenant: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }
  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(all_tenants))
    .routes(routes!(get_tenant_by_client_id))
    .routes(routes!(get_tenant_by_id))
    .routes(routes!(create_tenant))
    .routes(routes!(update_tenant))
    .routes(routes!(delete_tenant))
    .with_state(state)
}
