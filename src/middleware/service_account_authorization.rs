use std::collections::HashMap;

use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use serde_json::json;

use super::claims::{
  parse_jwt, parse_jwt_no_validation, BasicClaims, TOKEN_SUB_TYPE_SERVICE_ACCOUNT,
  TOKEN_TYPE_BEARER,
};
use crate::{
  core::{
    error::{Errors, INVALID, REQUIRED_ERROR},
    openapi::AUTHORIZATION_HEADER,
  },
  repository::{
    service_account::{get_service_account_by_id, ServiceAccountRow},
    tenent::get_tenent_by_id,
  },
  router::RouterState,
};

pub struct ServiceAccountAuthorization(pub ServiceAccountRow);

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
            return Err(Errors::unauthorized().with_error(
              AUTHORIZATION_HEADER,
              (
                REQUIRED_ERROR,
                HashMap::from([("in".to_owned(), json!("header"))]),
              ),
            ));
          }
          &authorization_string[(TOKEN_TYPE_BEARER.len() + 1)..]
        }
        Err(e) => {
          log::error!("invalid authorization header is missing: {}", e);
          return Err(Errors::unauthorized().with_error(
            AUTHORIZATION_HEADER,
            (
              REQUIRED_ERROR,
              HashMap::from([("in".to_owned(), json!("header"))]),
            ),
          ));
        }
      };
      let maybe_invalid_token = match parse_jwt_no_validation::<BasicClaims>(authorization_string) {
        Ok(maybe_invalid_token) => maybe_invalid_token,
        Err(e) => {
          log::error!("invalid authorization failed to check header: {}", e);
          return Err(Errors::unauthorized().with_error(
            AUTHORIZATION_HEADER,
            (INVALID, HashMap::from([("in".to_owned(), json!("header"))])),
          ));
        }
      };
      if maybe_invalid_token.claims.kind != TOKEN_TYPE_BEARER
        || maybe_invalid_token.claims.sub_kind != TOKEN_SUB_TYPE_SERVICE_ACCOUNT
      {
        return Err(Errors::unauthorized().with_error(
          AUTHORIZATION_HEADER,
          (INVALID, HashMap::from([("in".to_owned(), json!("header"))])),
        ));
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
          return Err(Errors::unauthorized().with_error(
            AUTHORIZATION_HEADER,
            (INVALID, HashMap::from([("in".to_owned(), json!("header"))])),
          ));
        }
        None => {
          log::error!("invalid authorization kid is missing");
          return Err(Errors::unauthorized().with_error(
            AUTHORIZATION_HEADER,
            (INVALID, HashMap::from([("in".to_owned(), json!("header"))])),
          ));
        }
      };
      let tenent = match get_tenent_by_id(&router_state.pool, tenent_id).await {
        Ok(Some(tenent)) => tenent,
        Ok(None) => {
          log::error!("invalid authorization tenent not found by app");
          return Err(Errors::unauthorized().with_error(
            AUTHORIZATION_HEADER,
            (INVALID, HashMap::from([("in".to_owned(), json!("header"))])),
          ));
        }
        Err(e) => {
          log::error!("invalid authorization token is invalid: {}", e);
          return Err(Errors::unauthorized().with_error(
            AUTHORIZATION_HEADER,
            (INVALID, HashMap::from([("in".to_owned(), json!("header"))])),
          ));
        }
      };
      let token_data = match parse_jwt::<BasicClaims>(authorization_string, &tenent) {
        Ok(token_data) => token_data,
        Err(e) => {
          log::error!("invalid authorization failed to parse claims: {}", e);
          return Err(Errors::unauthorized().with_error(
            AUTHORIZATION_HEADER,
            (INVALID, HashMap::from([("in".to_owned(), json!("header"))])),
          ));
        }
      };
      match get_service_account_by_id(&router_state.pool, token_data.claims.sub).await {
        Ok(Some(service_account)) => {
          if !service_account.is_active() {
            log::error!("invalid authorization service_account is not active");
            return Err(Errors::unauthorized().with_error(
              AUTHORIZATION_HEADER,
              (INVALID, HashMap::from([("in".to_owned(), json!("header"))])),
            ));
          }
          return Ok(Self(service_account));
        }
        Ok(None) => {
          return Err(Errors::unauthorized().with_error(
            AUTHORIZATION_HEADER,
            (INVALID, HashMap::from([("in".to_owned(), json!("header"))])),
          ))
        }
        Err(e) => {
          log::error!(
            "invalid authorization service_account not found for sub: {}",
            e
          );
          return Err(Errors::unauthorized().with_error(
            AUTHORIZATION_HEADER,
            (INVALID, HashMap::from([("in".to_owned(), json!("header"))])),
          ));
        }
      }
    }
    Err(Errors::unauthorized().with_error(
      AUTHORIZATION_HEADER,
      (
        REQUIRED_ERROR,
        HashMap::from([("in".to_owned(), json!("header"))]),
      ),
    ))
  }
}
