use crate::{
  core::{
    encryption,
    error::{Errors, InternalError, INTERNAL_ERROR, NOT_ALLOWED_ERROR, NOT_FOUND_ERROR},
  },
  middleware::{json::Json, service_account_authorization::ServiceAccountAuthorization},
  model::{
    service_account::{
      CreateServiceAccount, ServiceAccount, ServiceAccountPagination, UpdateServiceAccount,
    },
    util::{ApplicationId, OffsetAndLimit, Pagination},
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

pub const SERVICE_ACCOUNT_TAG: &str = "service-account";

#[utoipa::path(
  get,
  path = "/service-accounts",
  tags = [SERVICE_ACCOUNT_TAG],
  params(
    OffsetAndLimit,
    ApplicationId,
  ),
  responses(
    (status = 200, content_type = "application/json", body = ServiceAccountPagination),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn all_service_accounts(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Query(query): Query<OffsetAndLimit>,
  Query(application_id): Query<ApplicationId>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("view-service-accounts", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let rows = match repository::service_account::get_service_accounts(
    &state.pool,
    application_id,
    query.limit,
    query.offset,
  )
  .await
  {
    Ok(rows) => rows,
    Err(e) => {
      log::error!("error getting service_accounts: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let service_accounts = rows
    .into_iter()
    .map(ServiceAccount::from)
    .collect::<Vec<_>>();

  axum::Json(Pagination {
    has_more: query
      .limit
      .map(|limit| service_accounts.len() == limit)
      .unwrap_or(false),
    items: service_accounts,
  })
  .into_response()
}

#[utoipa::path(
  get,
  path = "/service-accounts/{service_account_id}",
  tags = [SERVICE_ACCOUNT_TAG],
  params(
    ("service_account_id" = i64, Path, description = "ServiceAccount ID"),
    ApplicationId,
  ),
  responses(
    (status = 200, content_type = "application/json", body = ServiceAccount),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn get_service_account_by_id(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path(service_account_id): Path<i64>,
  Query(application_id): Query<ApplicationId>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("view-service-accounts", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let row = match repository::service_account::get_service_account_by_id(
    &state.pool,
    application_id,
    service_account_id,
  )
  .await
  {
    Ok(Some(row)) => row,
    Ok(None) => {
      return InternalError::not_found()
        .with_error("service_account", NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error getting service_accounts: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let service_account = ServiceAccount::from(row);
  axum::Json(service_account).into_response()
}

#[utoipa::path(
  post,
  path = "/service-accounts",
  tags = [SERVICE_ACCOUNT_TAG],
  request_body = CreateServiceAccount,
  params(
    ApplicationId,
  ),
  responses(
    (status = 201, content_type = "application/json", body = ServiceAccount),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn create_service_account(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Query(application_id): Query<ApplicationId>,
  Json(payload): Json<CreateServiceAccount>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("view-service-accounts", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let is_admin = payload.admin.unwrap_or(false);
  if is_admin && !service_account.is_admin() {
    return InternalError::unauthorized()
      .with_error("create-admin-service-account", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let client_id = payload.client_id.unwrap_or_else(uuid::Uuid::new_v4);
  let client_secret = payload.client_secret.unwrap_or_else(uuid::Uuid::new_v4);
  let encrypted_client_secret =
    match encryption::encrypt_password(state.config.as_ref(), &client_secret.to_string()) {
      Ok(encrypted_client_secret) => encrypted_client_secret,
      Err(e) => {
        log::error!("error encrypting client_secret: {}", e);
        return InternalError::internal_error()
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    };
  let row = match repository::service_account::create_service_account(
    &state.pool,
    application_id,
    repository::service_account::CreateServiceAccount {
      name: payload.name,
      client_id: client_id.to_string(),
      encrypted_client_secret,
      admin: is_admin,
    },
  )
  .await
  {
    Ok(row) => row,
    Err(e) => {
      log::error!("error creating service_account: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let mut service_account = ServiceAccount::from(row);
  service_account.client_secret = Some(client_secret);
  axum::Json(service_account).into_response()
}

#[utoipa::path(
  put,
  path = "/service-accounts/{service_account_id}",
  tags = [SERVICE_ACCOUNT_TAG],
  request_body = UpdateServiceAccount,
  params(
    ("service_account_id" = i64, Path, description = "ServiceAccount ID"),
    ApplicationId
  ),
  responses(
    (status = 201, content_type = "application/json", body = ServiceAccount),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn update_service_account(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path(service_account_id): Path<i64>,
  Query(application_id): Query<ApplicationId>,
  Json(payload): Json<UpdateServiceAccount>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("view-service-accounts", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let mut params = repository::service_account::UpdateServiceAccount::default();
  params.admin = payload.admin;
  if params.admin == Some(true) && !service_account.is_admin() {
    return InternalError::unauthorized()
      .with_error("update-admin-service-account", NOT_ALLOWED_ERROR)
      .into_response();
  }
  if let Some(client_id) = payload.client_id {
    params.client_id = Some(client_id.to_string().to_owned());
  }
  if let Some(client_secret) = payload.client_secret {
    let encrypted_client_secret =
      match encryption::encrypt_password(state.config.as_ref(), &client_secret.to_string()) {
        Ok(encrypted_client_secret) => encrypted_client_secret,
        Err(e) => {
          log::error!("error encrypting client_secret: {}", e);
          return InternalError::internal_error()
            .with_application_error(INTERNAL_ERROR)
            .into_response();
        }
      };
    params.encrypted_client_secret = Some(encrypted_client_secret);
  }
  let row = match repository::service_account::update_service_account(
    &state.pool,
    application_id,
    service_account_id,
    params,
  )
  .await
  {
    Ok(Some(row)) => row,
    Ok(None) => {
      return InternalError::not_found()
        .with_error(SERVICE_ACCOUNT_TAG, NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error creating service_account: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let mut service_account = ServiceAccount::from(row);
  if let Some(client_secret) = payload.client_secret {
    service_account.client_secret = Some(client_secret);
  }
  axum::Json(service_account).into_response()
}

#[utoipa::path(
  delete,
  path = "/service-accounts/{service_account_id}",
  tags = [SERVICE_ACCOUNT_TAG],
  params(
    ("service_account_id" = i64, Path, description = "ServiceAccount ID"),
    ApplicationId
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
pub async fn delete_service_account(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path(service_account_id): Path<i64>,
  Query(application_id): Query<ApplicationId>,
) -> impl IntoResponse {
  let application_id = application_id
    .application_id
    .unwrap_or(service_account.application_id);
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("view-service-accounts", NOT_ALLOWED_ERROR)
      .into_response();
  }
  match repository::service_account::delete_service_account(
    &state.pool,
    application_id,
    service_account_id,
  )
  .await
  {
    Ok(Some(_)) => {}
    Ok(None) => {
      return InternalError::not_found()
        .with_error(SERVICE_ACCOUNT_TAG, NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error creating service_account: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }
  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(all_service_accounts))
    .routes(routes!(get_service_account_by_id))
    .routes(routes!(create_service_account))
    .routes(routes!(update_service_account))
    .routes(routes!(delete_service_account))
    .with_state(state)
}
