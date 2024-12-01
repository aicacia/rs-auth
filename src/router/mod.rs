pub mod current_user;
pub mod current_user_totp;
pub mod mfa;
pub mod oauth2;
pub mod openapi;
pub mod register;
pub mod token;
pub mod user;
pub mod util;

use axum::Router;
use sqlx::AnyPool;
use tower_http::cors::CorsLayer;
use utoipa::{OpenApi, openapi::Server};

use crate::core::{
  config::get_config,
  error::{ErrorMessage, ErrorMessages, Errors},
  openapi::SecurityAddon,
};

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
    (path = "/", api = current_user_totp::ApiDoc),
    (path = "/", api = current_user::ApiDoc),
    (path = "/", api = mfa::ApiDoc),
    (path = "/", api = oauth2::ApiDoc),
    (path = "/", api = openapi::ApiDoc),
    (path = "/", api = register::ApiDoc),
    (path = "/", api = token::ApiDoc),
    (path = "/", api = user::ApiDoc),
    (path = "/", api = util::ApiDoc),
  ),
  components(
    schemas(
      ErrorMessage,
      ErrorMessages,
      Errors,
    )
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
    .merge(current_user_totp::create_router(state.clone()))
    .merge(current_user::create_router(state.clone()))
    .merge(mfa::create_router(state.clone()))
    .merge(oauth2::create_router(state.clone()))
    .merge(openapi::create_router(doc))
    .merge(register::create_router(state.clone()))
    .merge(token::create_router(state.clone()))
    .merge(user::create_router(state.clone()))
    .merge(util::create_router(state.clone()))
    .layer(CorsLayer::very_permissive())
    .layer(
      tower_http::trace::TraceLayer::new_for_http().make_span_with(
        |request: &axum::http::Request<_>| {
          let matched_path = request
            .extensions()
            .get::<axum::extract::MatchedPath>()
            .map(axum::extract::MatchedPath::as_str);

          tracing::info_span!(
            "http",
            method = ?request.method(),
            path = matched_path,
          )
        },
      ),
    )
}
