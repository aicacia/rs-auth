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
use sqlx::AnyPool;
use tower_http::cors::CorsLayer;
use utoipa::{openapi::Server, OpenApi};

use crate::core::{config::get_config, openapi::SecurityAddon};

#[derive(Clone)]
pub struct RouterState {
  pub pool: AnyPool,
}

unsafe impl Send for RouterState {}
unsafe impl Sync for RouterState {}

#[derive(OpenApi)]
#[openapi(
  info(license(name = "MIT OR Apache-2.0", identifier = "https://spdx.org/licenses/MIT.html")),
  nest(
    (path = "/", api = current_user::ApiDoc),
    (path = "/", api = current_user_config::ApiDoc),
    (path = "/", api = current_user_email::ApiDoc),
    (path = "/", api = current_user_phone_number::ApiDoc),
    (path = "/", api = current_user_totp::ApiDoc),
    (path = "/", api = jwt::ApiDoc),
    (path = "/", api = mfa::ApiDoc),
    (path = "/", api = oauth2::ApiDoc),
    (path = "/", api = openapi::ApiDoc),
    (path = "/", api = register::ApiDoc),
    (path = "/", api = service_account::ApiDoc),
    (path = "/", api = tenant_oauth2_provider::ApiDoc),
    (path = "/", api = tenant::ApiDoc),
    (path = "/", api = token::ApiDoc),
    (path = "/", api = user::ApiDoc),
    (path = "/", api = user_email::ApiDoc),
    (path = "/", api = user_phone_number::ApiDoc),
    (path = "/", api = util::ApiDoc),
  ),
  modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub fn create_router(state: RouterState) -> Router {
  let config = get_config();

  let mut doc = ApiDoc::openapi();
  doc
    .servers
    .get_or_insert(Vec::default())
    .push(Server::new(config.server.url.clone()));

  Router::new()
    .merge(current_user::create_router(state.clone()))
    .merge(current_user_config::create_router(state.clone()))
    .merge(current_user_email::create_router(state.clone()))
    .merge(current_user_phone_number::create_router(state.clone()))
    .merge(current_user_totp::create_router(state.clone()))
    .merge(jwt::create_router(state.clone()))
    .merge(mfa::create_router(state.clone()))
    .merge(oauth2::create_router(state.clone()))
    .merge(openapi::create_router(doc))
    .merge(register::create_router(state.clone()))
    .merge(service_account::create_router(state.clone()))
    .merge(tenant_oauth2_provider::create_router(state.clone()))
    .merge(tenant::create_router(state.clone()))
    .merge(token::create_router(state.clone()))
    .merge(user::create_router(state.clone()))
    .merge(user_email::create_router(state.clone()))
    .merge(user_phone_number::create_router(state.clone()))
    .merge(util::create_router(state.clone()))
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
}
