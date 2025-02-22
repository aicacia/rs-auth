use std::str::FromStr;

use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use serde::de::DeserializeOwned;

use super::claims::{parse_jwt, parse_jwt_no_validation, BasicClaims, TOKEN_TYPE_BEARER};
use crate::{
  core::{
    error::{InternalError, INVALID_ERROR, PARSE_ERROR, REQUIRED_ERROR},
    openapi::AUTHORIZATION_HEADER,
  },
  repository::tenant::{get_tenant_by_id, TenantRow},
  router::RouterState,
};

pub struct Authorization {
  pub claims: BasicClaims,
  pub tenant: TenantRow,
}

impl<S> FromRequestParts<S> for Authorization
where
  RouterState: FromRef<S>,
  S: Send + Sync,
{
  type Rejection = InternalError;

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let router_state = RouterState::from_ref(state);

    if let Some(authorization_header_value) = parts.headers.get(AUTHORIZATION_HEADER) {
      let authorization_string = match authorization_header_value.to_str() {
        Ok(authorization_string) => {
          if authorization_string.len() < TOKEN_TYPE_BEARER.len() + 1 {
            log::error!("invalid authorization header is invalid");
            return Err(
              InternalError::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR),
            );
          }
          &authorization_string[(TOKEN_TYPE_BEARER.len() + 1)..]
        }
        Err(e) => {
          log::error!("invalid authorization header is invalid: {}", e);
          return Err(
            InternalError::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR),
          );
        }
      };
      let (tenant, token_data) =
        parse_authorization(&router_state.pool, authorization_string).await?;
      return Ok(Self {
        claims: token_data.claims,
        tenant,
      });
    }
    Err(InternalError::unauthorized().with_error(AUTHORIZATION_HEADER, REQUIRED_ERROR))
  }
}

pub async fn parse_authorization<T>(
  pool: &sqlx::AnyPool,
  authorization_string: &str,
) -> Result<(TenantRow, jsonwebtoken::TokenData<T>), InternalError>
where
  T: DeserializeOwned,
{
  let maybe_invalid_token = match parse_jwt_no_validation::<T>(authorization_string) {
    Ok(maybe_invalid_token) => maybe_invalid_token,
    Err(e) => {
      log::error!("invalid authorization failed to check header: {}", e);
      return Err(InternalError::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
    }
  };
  let ApplicationIdTenantId {
    application_id,
    tenant_id,
  } = match maybe_invalid_token
    .header
    .kid
    .as_ref()
    .map(String::as_str)
    .map(FromStr::from_str)
  {
    Some(Ok(application_id_tenant_id)) => application_id_tenant_id,
    Some(Err(e)) => {
      log::error!("invalid authorization failed to parse kid: {}", e);
      return Err(InternalError::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
    }
    None => {
      log::error!("invalid authorization kid is missing");
      return Err(InternalError::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
    }
  };
  let tenant = match get_tenant_by_id(pool, application_id, tenant_id).await {
    Ok(Some(tenant)) => tenant,
    Ok(None) => {
      log::error!("invalid authorization tenant not found by app");
      return Err(InternalError::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
    }
    Err(e) => {
      log::error!("invalid authorization token is invalid: {}", e);
      return Err(InternalError::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
    }
  };
  let token_data = match parse_jwt::<T>(authorization_string, &tenant) {
    Ok(token_data) => token_data,
    Err(e) => {
      log::error!("invalid authorization failed to parse claims: {}", e);
      return Err(InternalError::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
    }
  };
  Ok((tenant, token_data))
}

pub struct ApplicationIdTenantId {
  application_id: i64,
  tenant_id: i64,
}

impl ApplicationIdTenantId {
  pub fn new_kid(application_id: i64, tenant_id: i64) -> String {
    format!("{}-{}", application_id, tenant_id)
  }
}

impl FromStr for ApplicationIdTenantId {
  type Err = InternalError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.split_once('-') {
      Some((application_id, tenant_id)) => Ok(Self {
        application_id: application_id
          .parse()
          .map_err(|_| InternalError::not_found().with_error("kid", PARSE_ERROR))?,
        tenant_id: tenant_id
          .parse()
          .map_err(|_| InternalError::not_found().with_error("kid", PARSE_ERROR))?,
      }),
      None => Err(InternalError::not_found().with_error("kid", PARSE_ERROR)),
    }
  }
}
