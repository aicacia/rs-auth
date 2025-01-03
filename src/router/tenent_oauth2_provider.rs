use crate::{
  core::error::{Errors, ALREADY_EXISTS_ERROR, INTERNAL_ERROR, NOT_ALLOWED_ERROR, NOT_FOUND_ERROR},
  middleware::{json::Json, service_account_authorization::ServiceAccountAuthorization},
  model::tenent_oauth2_provider::{
    CreateTenentOAuth2Provider, TenentOAuth2Provider, UpdateTenentOAuth2Provider,
  },
  repository,
};

use axum::{
  extract::{Path, State},
  response::IntoResponse,
  routing::{delete, post, put},
  Router,
};
use http::StatusCode;
use utoipa::OpenApi;

use super::RouterState;

#[derive(OpenApi)]
#[openapi(
  paths(
    create_tenent_oauth2_provider, 
    update_tenent_oauth2_provider,
    delete_tenent_oauth2_provider
  ),
  tags(
    (name = "oauth2-provider", description = "OAuth2 Provider endpoints"),
  )
)]
pub struct ApiDoc;

#[utoipa::path(
  post,
  path = "tenents/{tenent_id}/oauth2-providers",
  tags = ["oauth2-provider"],
  request_body = CreateTenentOAuth2Provider,
  params(
    ("tenent_id" = i64, Path, description = "Tenent ID")
  ),
  responses(
    (status = 201, content_type = "application/json", body = TenentOAuth2Provider),
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
pub async fn create_tenent_oauth2_provider(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path(tenent_id): Path<i64>,
  Json(payload): Json<CreateTenentOAuth2Provider>,
) -> impl IntoResponse {
  let mut params =
    match repository::tenent_oauth2_provider::CreateTenentOAuth2Provider::new(&payload.provider) {
      Some(params) => params,
      None => {
        return Errors::from(StatusCode::NOT_IMPLEMENTED)
          .with_error("provider", NOT_ALLOWED_ERROR)
          .into_response();
      }
    };

  params.client_id = payload.client_id;
  params.client_secret = payload.client_secret;
  params.redirect_url = payload.redirect_url;
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

  let tenent = match repository::tenent_oauth2_provider::create_tenent_oauth2_provider(&state.pool, tenent_id, params).await {
    Ok(tenent) => tenent,
    Err(e) => {
      if e.to_string().to_lowercase().contains("unique constraint") {
        return Errors::from(StatusCode::CONFLICT)
          .with_error("oauth2-provider", ALREADY_EXISTS_ERROR)
          .into_response();
      }
      log::error!("error creating tenent OAuth2 provider: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  axum::Json(TenentOAuth2Provider::from(tenent)).into_response()
}

#[utoipa::path(
  put,
  path = "tenents/{tenent_id}/oauth2-providers/{tenent_oauht2_provider_id}",
  tags = ["oauth2-provider"],
  request_body = UpdateTenentOAuth2Provider,
  params(
    ("tenent_id" = i64, Path, description = "Tenent ID"),
    ("tenent_oauht2_provider_id" = i64, Path, description = "OAuth2 Provider ID"),
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
pub async fn update_tenent_oauth2_provider(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path((tenent_id, tenent_oauht2_provider_id)): Path<(i64, i64)>,
  Json(payload): Json<UpdateTenentOAuth2Provider>,
) -> impl IntoResponse {
  match repository::tenent_oauth2_provider::update_tenent_oauth2_provider(
    &state.pool,
    tenent_id,
    tenent_oauht2_provider_id,
    repository::tenent_oauth2_provider::UpdateTenentOAuth2Provider {
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
    Ok(Some(_)) => {},
    Ok(None) => {
      return Errors::not_found().with_error("tenent-oauth2-provider", NOT_FOUND_ERROR).into_response();
    }
    Err(e) => {
      log::error!("error updating tenent OAuth2 provider: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  };
  (StatusCode::NO_CONTENT, ()).into_response()
}

#[utoipa::path(
  delete,
  path = "tenents/{tenent_id}/oauth2-providers/{tenent_oauht2_provider_id}",
  tags = ["oauth2-provider"],
  params(
    ("tenent_id" = i64, Path, description = "Tenent ID"),
    ("tenent_oauht2_provider_id" = i64, Path, description = "OAuth2 Provider ID"),
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
pub async fn delete_tenent_oauth2_provider(
  State(state): State<RouterState>,
  ServiceAccountAuthorization { .. }: ServiceAccountAuthorization,
  Path((tenent_id, tenent_oauht2_provider_id)): Path<(i64, i64)>
) -> impl IntoResponse {
  match repository::tenent_oauth2_provider::delete_tenent_oauth2_provider(&state.pool, tenent_id, tenent_oauht2_provider_id).await {
    Ok(Some(_)) => {},
    Ok(None) => {
      return Errors::not_found().with_error("tenent-oauth2-provider", NOT_FOUND_ERROR).into_response();
    }
    Err(e) => {
      log::error!("error deleting tenent OAuth2 provider: {}", e);
      return Errors::internal_error()
        .with_application_error(INTERNAL_ERROR)
        .into_response();
    }
  }
  (StatusCode::NO_CONTENT, ()).into_response()
}

pub fn create_router(state: RouterState) -> Router {
  Router::new()
    .route(
      "/tenents/{tenent_id}/oauth2-providers",
      post(create_tenent_oauth2_provider),
    )
    .route(
      "/tenents/{tenent_id}/oauth2-providers/{tenent_oauht2_provider_id}",
      put(update_tenent_oauth2_provider),
    )
    .route(
      "/tenents/{tenent_id}/oauth2-providers/{tenent_oauht2_provider_id}",
      delete(delete_tenent_oauth2_provider),
    )
    .with_state(state)
}
