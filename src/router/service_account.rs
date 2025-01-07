use crate::{
  core::{
    encryption,
    error::{Errors, INTERNAL_ERROR, NOT_FOUND_ERROR},
  },
  middleware::{json::Json, service_account_authorization::ServiceAccountAuthorization},
  model::{
    service_account::{CreateServiceAccount, ServiceAccount, UpdateServiceAccount},
    util::{OffsetAndLimit, Pagination},
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
  ),
  responses(
    (status = 200, content_type = "application/json", body = Pagination<ServiceAccount>),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn all_service_accounts(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Query(query): Query<OffsetAndLimit>,
) -> impl IntoResponse {
  let rows =
    match repository::service_account::get_service_accounts(&state.pool, query.limit, query.offset)
      .await
    {
      Ok(rows) => rows,
      Err(e) => {
        log::error!("error getting service_accounts: {}", e);
        return Errors::internal_error()
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
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(service_account_id): Path<i64>,
) -> impl IntoResponse {
  let row =
    match repository::service_account::get_service_account_by_id(&state.pool, service_account_id)
      .await
    {
      Ok(Some(row)) => row,
      Ok(None) => {
        return Errors::not_found()
          .with_error("service_account", NOT_FOUND_ERROR)
          .into_response()
      }
      Err(e) => {
        log::error!("error getting service_accounts: {}", e);
        return Errors::internal_error()
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
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Json(payload): Json<CreateServiceAccount>,
) -> impl IntoResponse {
  let client_id = payload.client_id.unwrap_or_else(uuid::Uuid::new_v4);
  let client_secret = payload.client_secret.unwrap_or_else(uuid::Uuid::new_v4);
  let encrypted_client_secret = match encryption::encrypt_password(&client_secret.to_string()) {
    Ok(encrypted_client_secret) => encrypted_client_secret,
    Err(e) => {
      log::error!("error encrypting client_secret: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let row = match repository::service_account::create_service_account(
    &state.pool,
    repository::service_account::CreateServiceAccount {
      name: payload.name,
      client_id: client_id.to_string(),
      encrypted_client_secret,
    },
  )
  .await
  {
    Ok(row) => row,
    Err(e) => {
      log::error!("error creating service_account: {}", e);
      return Errors::internal_error()
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
    ("service_account_id" = i64, Path, description = "ServiceAccount ID")
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
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(service_account_id): Path<i64>,
  Json(payload): Json<UpdateServiceAccount>,
) -> impl IntoResponse {
  let mut params = repository::service_account::UpdateServiceAccount::default();
  if let Some(client_id) = payload.client_id {
    params.client_id = Some(client_id.to_string().to_owned());
  }
  if let Some(client_secret) = payload.client_secret {
    let encrypted_client_secret = match encryption::encrypt_password(&client_secret.to_string()) {
      Ok(encrypted_client_secret) => encrypted_client_secret,
      Err(e) => {
        log::error!("error encrypting client_secret: {}", e);
        return Errors::internal_error()
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    };
    params.encrypted_client_secret = Some(encrypted_client_secret);
  }
  let row = match repository::service_account::update_service_account(
    &state.pool,
    service_account_id,
    params,
  )
  .await
  {
    Ok(Some(row)) => row,
    Ok(None) => {
      return Errors::not_found()
        .with_error(SERVICE_ACCOUNT_TAG, NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error creating service_account: {}", e);
      return Errors::internal_error()
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
    ("service_account_id" = i64, Path, description = "ServiceAccount ID")
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
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(service_account_id): Path<i64>,
) -> impl IntoResponse {
  match repository::service_account::delete_service_account(&state.pool, service_account_id).await {
    Ok(Some(_)) => {}
    Ok(None) => {
      return Errors::not_found()
        .with_error(SERVICE_ACCOUNT_TAG, NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error creating service_account: {}", e);
      return Errors::internal_error()
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
    .with_state(state)
}
