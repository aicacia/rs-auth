pub mod current_user;
pub mod current_user_config;
pub mod current_user_email;
pub mod current_user_phone_number;
pub mod current_user_totp;
pub mod jwt;
pub mod mfa;
pub mod oauth2;
pub mod openapi;
pub mod register;
pub mod service_account;
pub mod tenant;
pub mod tenant_oauth2_provider;
pub mod token;
pub mod user;
pub mod user_email;
pub mod user_phone_number;
pub mod util;

use axum::Router;
use current_user::CURRENT_USER_TAG;
use jwt::JWT_TAG;
use mfa::MFA_TAG;
use oauth2::OAUTH2_TAG;
use openapi::OPENAPI_TAG;
use register::REGISTER_TAG;
use service_account::SERVICE_ACCOUNT_TAG;
use sqlx::AnyPool;
use tenant::TENANT_TAG;
use tenant_oauth2_provider::OAUTH2_PROVIDER_TAG;
use token::TOKEN_TAG;
use tower_http::cors::CorsLayer;
use user::USER_TAG;
use util::UTIL_TAG;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::core::openapi::{SecurityAddon, ServersAddon};

#[derive(Clone)]
pub struct RouterState {
  pub pool: AnyPool,
}

unsafe impl Send for RouterState {}
unsafe impl Sync for RouterState {}

#[derive(OpenApi)]
#[openapi(
  info(license(name = "MIT OR Apache-2.0", identifier = "https://spdx.org/licenses/MIT.html")),
  tags(
    (name = CURRENT_USER_TAG, description = "Current user endpoints"),
    (name = JWT_TAG, description = "JSON Web Token endpoints"),
    (name = MFA_TAG, description = "Multi-factor authentication endpoints"),
    (name = UTIL_TAG, description = "Utility endpoints"),
    (name = OAUTH2_TAG, description = "OAuth2 endpoints"),
    (name = OPENAPI_TAG, description = "OpenApi endpoints"),
    (name = REGISTER_TAG, description = "Register endpoints"),
    (name = SERVICE_ACCOUNT_TAG, description = "Service Account endpoints"),
    (name = OAUTH2_PROVIDER_TAG, description = "OAuth2 Provider endpoints"),
    (name = TENANT_TAG, description = "Tenant endpoints"),
    (name = TOKEN_TAG, description = "Token endpoints"),
    (name = USER_TAG, description = "User endpoints"),
  ),
  modifiers(&SecurityAddon, &ServersAddon)
)]
pub struct ApiDoc;

pub fn create_router(state: RouterState) -> Router {
  let open_api_router = OpenApiRouter::with_openapi(ApiDoc::openapi())
    .merge(current_user::create_router(state.clone()))
    .merge(current_user_config::create_router(state.clone()))
    .merge(current_user_email::create_router(state.clone()))
    .merge(current_user_phone_number::create_router(state.clone()))
    .merge(current_user_totp::create_router(state.clone()))
    .merge(jwt::create_router(state.clone()))
    .merge(mfa::create_router(state.clone()))
    .merge(oauth2::create_router(state.clone()))
    .merge(register::create_router(state.clone()))
    .merge(service_account::create_router(state.clone()))
    .merge(tenant_oauth2_provider::create_router(state.clone()))
    .merge(tenant::create_router(state.clone()))
    .merge(token::create_router(state.clone()))
    .merge(user::create_router(state.clone()))
    .merge(user_email::create_router(state.clone()))
    .merge(user_phone_number::create_router(state.clone()))
    .merge(util::create_router(state.clone()));

  let openapi = open_api_router.get_openapi().clone();
  open_api_router
    .merge(openapi::create_router(openapi))
    .layer(CorsLayer::very_permissive())
    .layer(
      tower_http::trace::TraceLayer::new_for_http().make_span_with(
        |request: &axum::http::Request<_>| {
          tracing::info_span!(
            "http",
            method = ?request.method(),
            path = ?request.uri(),
          )
        },
      ),
    )
    .into()
}
