use crate::{
  core::error::{
    Errors, InternalError, ALREADY_EXISTS_ERROR, INTERNAL_ERROR, NOT_ALLOWED_ERROR, NOT_FOUND_ERROR,
  },
  middleware::{json::Json, service_account_authorization::ServiceAccountAuthorization},
  model::{
    tenant_oauth2_provider::{
      CreateTenantOAuth2Provider, TenantOAuth2Provider, UpdateTenantOAuth2Provider,
    },
    util::ApplicationId,
  },
  repository,
};

use axum::{
  extract::{Path, Query, State},
  response::IntoResponse,
};
use http::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::RouterState;

pub const TENANT_OAUTH2_PROVIDER_TAG: &str = "tenant-oauth2-provider";

#[utoipa::path(
  post,
  path = "/tenants/{tenant_id}/oauth2-providers",
  tags = [TENANT_OAUTH2_PROVIDER_TAG],
  request_body = CreateTenantOAuth2Provider,
  params(
    ("tenant_id" = i64, Path, description = "Tenant ID"),
    ApplicationId
  ),
  responses(
    (status = 201, content_type = "application/json", body = TenantOAuth2Provider),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 409, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
    (status = 501, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn create_tenant_oauth2_provider(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path(tenant_id): Path<i64>,
  Query(application_id): Query<ApplicationId>,
  Json(payload): Json<CreateTenantOAuth2Provider>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("create-tenant-oauth2-providers", NOT_ALLOWED_ERROR)
      .into_response();
  }
  match repository::tenant::get_tenant_by_id(&state.pool, application_id, tenant_id).await {
    Ok(Some(..)) => {}
    Ok(None) => {
      return InternalError::not_found()
        .with_error("tenant", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error getting tenant: {e}");
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let mut params =
    match repository::tenant_oauth2_provider::CreateTenantOAuth2Provider::new(&payload.provider) {
      Some(params) => params,
      None => {
        return InternalError::from(StatusCode::NOT_IMPLEMENTED)
          .with_error("provider", NOT_ALLOWED_ERROR)
          .into_response();
      }
    };

  params.client_id = payload.client_id;
  params.client_secret = payload.client_secret;
  params.redirect_url = payload.redirect_url;
  params.active = payload.active.unwrap_or(true) as i64;
  if let Some(auth_url) = payload.auth_url {
    params.auth_url = auth_url;
  }
  if let Some(token_url) = payload.token_url {
    params.token_url = token_url;
  }
  if let Some(scope) = payload.scope {
    params.scope = scope;
  }
  if let Some(callback_url) = payload.callback_url {
    params.callback_url = Some(callback_url);
  }

  let tenant = match repository::tenant_oauth2_provider::create_tenant_oauth2_provider(
    &state.pool,
    tenant_id,
    params,
  )
  .await
  {
    Ok(tenant) => tenant,
    Err(e) => {
      if e.to_string().to_lowercase().contains("unique constraint") {
        return InternalError::from(StatusCode::CONFLICT)
          .with_error(TENANT_OAUTH2_PROVIDER_TAG, ALREADY_EXISTS_ERROR)
          .into_response();
      }
      log::error!("error creating tenant OAuth2 provider: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  axum::Json(TenantOAuth2Provider::from((state.config.as_ref(), tenant))).into_response()
}

#[utoipa::path(
  put,
  path = "/tenants/{tenant_id}/oauth2-providers/{tenant_oauht2_provider_id}",
  tags = [TENANT_OAUTH2_PROVIDER_TAG],
  request_body = UpdateTenantOAuth2Provider,
  params(
    ("tenant_id" = i64, Path, description = "Tenant ID"),
    ("tenant_oauht2_provider_id" = i64, Path, description = "OAuth2 Provider ID"),
    ApplicationId,
  ),
  responses(
    (status = 200, content_type = "application/json", body = TenantOAuth2Provider),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn update_tenant_oauth2_provider(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path((tenant_id, tenant_oauht2_provider_id)): Path<(i64, i64)>,
  Query(application_id): Query<ApplicationId>,
  Json(payload): Json<UpdateTenantOAuth2Provider>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("create-tenant-oauth2-providers", NOT_ALLOWED_ERROR)
      .into_response();
  }
  match repository::tenant::get_tenant_by_id(&state.pool, application_id, tenant_id).await {
    Ok(Some(..)) => {}
    Ok(None) => {
      return InternalError::not_found()
        .with_error("tenant", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error getting tenant: {e}");
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let tenant = match repository::tenant_oauth2_provider::update_tenant_oauth2_provider(
    &state.pool,
    tenant_id,
    tenant_oauht2_provider_id,
    repository::tenant_oauth2_provider::UpdateTenantOAuth2Provider {
      client_id: payload.client_id,
      client_secret: payload.client_secret,
      active: payload.active.map(Into::into),
      auth_url: payload.auth_url,
      token_url: payload.token_url,
      callback_url: payload.callback_url,
      redirect_url: payload.redirect_url,
      scope: payload.scope,
    },
  )
  .await
  {
    Ok(Some(tenant)) => tenant,
    Ok(None) => {
      return InternalError::not_found()
        .with_error("tenant-oauth2-provider", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error updating tenant OAuth2 provider: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };

  axum::Json(TenantOAuth2Provider::from((state.config.as_ref(), tenant))).into_response()
}

#[utoipa::path(
  delete,
  path = "/tenants/{tenant_id}/oauth2-providers/{tenant_oauht2_provider_id}",
  tags = [TENANT_OAUTH2_PROVIDER_TAG],
  params(
    ("tenant_id" = i64, Path, description = "Tenant ID"),
    ("tenant_oauht2_provider_id" = i64, Path, description = "OAuth2 Provider ID"),
    ApplicationId,
  ),
  responses(
    (status = 204),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn delete_tenant_oauth2_provider(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path((tenant_id, tenant_oauht2_provider_id)): Path<(i64, i64)>,
  Query(application_id): Query<ApplicationId>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("create-tenant-oauth2-providers", NOT_ALLOWED_ERROR)
      .into_response();
  }
  match repository::tenant::get_tenant_by_id(&state.pool, application_id, tenant_id).await {
    Ok(Some(..)) => {}
    Ok(None) => {
      return InternalError::not_found()
        .with_error("tenant", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error getting tenant: {e}");
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  match repository::tenant_oauth2_provider::delete_tenant_oauth2_provider(
    &state.pool,
    tenant_id,
    tenant_oauht2_provider_id,
  )
  .await
  {
    Ok(Some(_)) => {}
    Ok(None) => {
      return InternalError::not_found()
        .with_error("tenant-oauth2-provider", NOT_FOUND_ERROR)
        .into_response();
    }
    Err(e) => {
      log::error!("error deleting tenant OAuth2 provider: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }
  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(
      create_tenant_oauth2_provider,
      update_tenant_oauth2_provider,
      delete_tenant_oauth2_provider
    ))
    .with_state(state)
}
