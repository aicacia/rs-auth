use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;

use super::{
  authorization::Authorization,
  claims::{TOKEN_SUB_TYPE_SERVICE_ACCOUNT, TOKEN_TYPE_BEARER},
};
use crate::{
  core::{
    error::{InternalError, INVALID_ERROR},
    openapi::AUTHORIZATION_HEADER,
  },
  repository::{
    service_account::{get_service_account_by_id, ServiceAccountRow},
    tenant::TenantRow,
  },
  router::RouterState,
};

pub struct ServiceAccountAuthorization {
  pub service_account: ServiceAccountRow,
  pub tenant: TenantRow,
  pub scopes: Vec<String>,
}

impl<S> FromRequestParts<S> for ServiceAccountAuthorization
where
  RouterState: FromRef<S>,
  S: Send + Sync,
{
  type Rejection = InternalError;

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let router_state = RouterState::from_ref(state);
    let authorization = Authorization::from_request_parts(parts, state).await?;

    if authorization.claims.r#type != TOKEN_TYPE_BEARER
      || authorization.claims.sub_type != TOKEN_SUB_TYPE_SERVICE_ACCOUNT
    {
      return Err(
        InternalError::unauthorized().with_error(AUTHORIZATION_HEADER, "invalid-token-type"),
      );
    }

    match get_service_account_by_id(
      &router_state.pool,
      authorization.claims.app,
      authorization.claims.sub,
    )
    .await
    {
      Ok(Some(service_account)) => {
        if !service_account.is_active() {
          log::error!("invalid authorization service_account is not active");
          return Err(
            InternalError::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR),
          );
        }
        return Ok(Self {
          service_account,
          tenant: authorization.tenant,
          scopes: authorization.claims.scopes,
        });
      }
      Ok(None) => {
        return Err(InternalError::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
      }
      Err(e) => {
        log::error!(
          "invalid authorization service_account not found for sub: {}",
          e
        );
        return Err(InternalError::unauthorized().with_error(AUTHORIZATION_HEADER, INVALID_ERROR));
      }
    }
  }
}
