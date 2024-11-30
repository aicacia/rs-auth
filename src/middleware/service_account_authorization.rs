use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;

use super::claims::{
  BasicClaims, TOKEN_SUB_TYPE_SERVICE_ACCOUNT, TOKEN_TYPE_BEARER, parse_jwt,
  parse_jwt_no_validation,
};
use crate::{
  core::{
    error::{Errors, INVALID_ERROR, REQUIRED_ERROR},
    openapi::AUTHORIZATION_HEADER,
  },
  repository::{
    service_account::{ServiceAccountRow, get_service_account_by_id},
    tenent::{TenentRow, get_tenent_by_id},
  },
  router::RouterState,
};

pub struct ServiceAccountAuthorization {
  pub service_account: ServiceAccountRow,
  pub tenent: TenentRow,
  pub scopes: Vec<String>,
}

impl<S> FromRequestParts<S> for ServiceAccountAuthorization
where
  RouterState: FromRef<S>,
  S: Send + Sync,
{
  type Rejection = Errors;

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let router_state = RouterState::from_ref(state);

    if let Some(authorization_header_value) = parts.headers.get(AUTHORIZATION_HEADER) {
      let authorization_string = match authorization_header_value.to_str() {
        Ok(authorization_string) => {
          if authorization_string.len() < TOKEN_TYPE_BEARER.len() + 1 {
            log::error!("invalid authorization header is missing");
            return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, REQUIRED_ERROR));
          }
          &authorization_string[(TOKEN_TYPE_BEARER.len() + 1)..]
        }
        Err(e) => {
          log::error!("invalid authorization header is missing: {}", e);
          return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, REQUIRED_ERROR));
        }
      };
      let maybe_invalid_token = match parse_jwt_no_validation::<BasicClaims>(authorization_string) {
        Ok(maybe_invalid_token) => maybe_invalid_token,
        Err(e) => {
          log::error!("invalid authorization failed to check header: {}", e);
          return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
        }
      };
      if maybe_invalid_token.claims.kind != TOKEN_TYPE_BEARER
        || maybe_invalid_token.claims.sub_kind != TOKEN_SUB_TYPE_SERVICE_ACCOUNT
      {
        return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
      }
      let tenent_id = match maybe_invalid_token
        .header
        .kid
        .as_ref()
        .map(String::as_str)
        .map(str::parse::<i64>)
      {
        Some(Ok(tenent_id)) => tenent_id,
        Some(Err(e)) => {
          log::error!("invalid authorization failed to parse kid: {}", e);
          return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
        }
        None => {
          log::error!("invalid authorization kid is missing");
          return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
        }
      };
      let tenent = match get_tenent_by_id(&router_state.pool, tenent_id).await {
        Ok(Some(tenent)) => tenent,
        Ok(None) => {
          log::error!("invalid authorization tenent not found by app");
          return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
        }
        Err(e) => {
          log::error!("invalid authorization token is invalid: {}", e);
          return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
        }
      };
      let token_data = match parse_jwt::<BasicClaims>(authorization_string, &tenent) {
        Ok(token_data) => token_data,
        Err(e) => {
          log::error!("invalid authorization failed to parse claims: {}", e);
          return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
        }
      };
      match get_service_account_by_id(&router_state.pool, token_data.claims.sub).await {
        Ok(Some(service_account)) => {
          if !service_account.is_active() {
            log::error!("invalid authorization service_account is not active");
            return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
          }
          return Ok(Self {
            service_account,
            tenent,
            scopes: token_data.claims.scopes,
          });
        }
        Ok(None) => {
          return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
        }
        Err(e) => {
          log::error!(
            "invalid authorization service_account not found for sub: {}",
            e
          );
          return Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
        }
      }
    }
    Err(Errors::unauthorized().with_error(AUTHORIZATION_HEADER, REQUIRED_ERROR))
  }
}
