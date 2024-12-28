use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;

use super::{
  authorization::Authorization,
  claims::{TOKEN_SUB_TYPE_USER, TOKEN_TYPE_BEARER},
};
use crate::{
  core::{
    error::{Errors, INVALID_ERROR},
    openapi::AUTHORIZATION_HEADER,
  },
  repository::{
    tenent::TenentRow,
    user::{UserRow, get_user_by_id},
  },
  router::RouterState,
};

pub struct UserAuthorization {
  pub user: UserRow,
  pub tenent: TenentRow,
  pub scopes: Vec<String>,
}

impl<S> FromRequestParts<S> for UserAuthorization
where
  RouterState: FromRef<S>,
  S: Send + Sync,
{
  type Rejection = Errors;

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let router_state = RouterState::from_ref(state);
    let authorization = Authorization::from_request_parts(parts, state).await?;

    if authorization.claims.kind != TOKEN_TYPE_BEARER
      || authorization.claims.sub_kind != TOKEN_SUB_TYPE_USER
    {
      return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, "invalid-token-type"));
    }

    match get_user_by_id(&router_state.pool, authorization.claims.sub).await {
      Ok(Some(user)) => {
        if !user.is_active() {
          log::error!("invalid authorization user is not active");
          return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
        }
        return Ok(Self {
          user,
          tenent: authorization.tenent,
          scopes: authorization.claims.scopes,
        });
      }
      Ok(None) => {
        return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
      }
      Err(e) => {
        log::error!("invalid authorization user not found for sub: {}", e);
        return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
      }
    }
  }
}