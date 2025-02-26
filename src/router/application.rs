use crate::{
  core::error::{Errors, InternalError, INTERNAL_ERROR, NOT_ALLOWED_ERROR, NOT_FOUND_ERROR},
  middleware::{json::Json, service_account_authorization::ServiceAccountAuthorization},
  model::{
    application::{Application, ApplicationPagination, CreateApplication, UpdateApplication},
    util::OffsetAndLimit,
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

pub const APPLICATION_TAG: &str = "application";

#[utoipa::path(
  get,
  path = "/applications",
  tags = [APPLICATION_TAG],
  params(
    OffsetAndLimit,
  ),
  responses(
    (status = 200, content_type = "application/json", body = ApplicationPagination),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn all_applications(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Query(query): Query<OffsetAndLimit>,
) -> impl IntoResponse {
  let rows = if service_account.is_admin() {
    match repository::application::get_applications(&state.pool, query.limit, query.offset).await {
      Ok(rows) => rows,
      Err(e) => {
        log::error!("error getting applications: {}", e);
        return InternalError::internal_error()
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    }
  } else {
    match repository::application::get_application_by_id(
      &state.pool,
      service_account.application_id,
    )
    .await
    {
      Ok(Some(row)) => vec![row],
      Ok(None) => {
        return InternalError::not_found()
          .with_error("application", NOT_FOUND_ERROR)
          .into_response()
      }
      Err(e) => {
        log::error!("error getting applications: {}", e);
        return InternalError::internal_error()
          .with_application_error(INTERNAL_ERROR)
          .into_response();
      }
    }
  };
  let applications = rows.into_iter().map(Application::from).collect::<Vec<_>>();

  axum::Json(ApplicationPagination {
    has_more: query
      .limit
      .map(|limit| applications.len() == limit)
      .unwrap_or(false),
    items: applications,
  })
  .into_response()
}

#[utoipa::path(
  get,
  path = "/applications/{application_id}",
  tags = [APPLICATION_TAG],
  params(
    ("application_id" = i64, Path, description = "Application ID"),
  ),
  responses(
    (status = 200, content_type = "application/json", body = Application),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 404, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn get_application_by_id(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path(application_id): Path<i64>,
) -> impl IntoResponse {
  if !service_account.is_admin() && service_account.application_id != application_id {
    return InternalError::unauthorized()
      .with_error("view-application", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let row = match repository::application::get_application_by_id(&state.pool, application_id).await
  {
    Ok(Some(row)) => row,
    Ok(None) => {
      return InternalError::not_found()
        .with_error("application", NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error getting applications: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  let application = Application::from(row);
  axum::Json(application).into_response()
}

#[utoipa::path(
  post,
  path = "/applications",
  tags = [APPLICATION_TAG],
  request_body = CreateApplication,
  responses(
    (status = 201, content_type = "application/json", body = Application),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn create_application(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Json(payload): Json<CreateApplication>,
) -> impl IntoResponse {
  if !service_account.is_admin() {
    return InternalError::unauthorized()
      .with_error("create-application", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let row = match repository::application::create_application(
    &state.pool,
    repository::application::CreateApplication { name: payload.name },
  )
  .await
  {
    Ok(row) => row,
    Err(e) => {
      log::error!("error creating application: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  axum::Json(Application::from(row)).into_response()
}

#[utoipa::path(
  put,
  path = "/applications/{application_id}",
  tags = [APPLICATION_TAG],
  request_body = UpdateApplication,
  params(
    ("application_id" = i64, Path, description = "Application ID")
  ),
  responses(
    (status = 201, content_type = "application/json", body = Application),
    (status = 400, content_type = "application/json", body = Errors),
    (status = 401, content_type = "application/json", body = Errors),
    (status = 500, content_type = "application/json", body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
pub async fn update_application(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path(application_id): Path<i64>,
  Json(payload): Json<UpdateApplication>,
) -> impl IntoResponse {
  if !service_account.is_admin() {
    return InternalError::unauthorized()
      .with_error("update-application", NOT_ALLOWED_ERROR)
      .into_response();
  }
  let row = match repository::application::update_application(
    &state.pool,
    application_id,
    repository::application::UpdateApplication { name: payload.name },
  )
  .await
  {
    Ok(Some(row)) => row,
    Ok(None) => {
      return InternalError::not_found()
        .with_error(APPLICATION_TAG, NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error creating application: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  axum::Json(Application::from(row)).into_response()
}

#[utoipa::path(
  delete,
  path = "/applications/{application_id}",
  tags = [APPLICATION_TAG],
  params(
    ("application_id" = i64, Path, description = "Application ID")
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
pub async fn delete_application(
  State(state): State<RouterState>,
  ServiceAccountAuthorization {
    service_account, ..
  }: ServiceAccountAuthorization,
  Path(application_id): Path<i64>,
) -> impl IntoResponse {
  if !service_account.is_admin() {
    return InternalError::unauthorized()
      .with_error("delete-application", NOT_ALLOWED_ERROR)
      .into_response();
  }
  match repository::application::delete_application(&state.pool, application_id).await {
    Ok(Some(_)) => {}
    Ok(None) => {
      return InternalError::not_found()
        .with_error(APPLICATION_TAG, NOT_FOUND_ERROR)
        .into_response()
    }
    Err(e) => {
      log::error!("error creating application: {}", e);
      return InternalError::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }
  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> OpenApiRouter {
  OpenApiRouter::new()
    .routes(routes!(all_applications))
    .routes(routes!(get_application_by_id))
    .routes(routes!(create_application))
    .routes(routes!(update_application))
    .routes(routes!(delete_application))
    .with_state(state)
}
