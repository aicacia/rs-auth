use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;

use crate::{
  core::{
    error::{InternalError, INVALID_ERROR, PARSE_ERROR, REQUIRED_ERROR},
    openapi::TENENT_ID_HEADER,
  },
  repository::tenant::{get_tenant_by_client_id, TenantRow},
  router::RouterState,
};

pub struct TenantId(pub TenantRow);

impl<S> FromRequestParts<S> for TenantId
where
  RouterState: FromRef<S>,
  S: Send + Sync,
{
  type Rejection = InternalError;

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let router_state = RouterState::from_ref(state);

    if let Some(id_header_value) = parts.headers.get(TENENT_ID_HEADER) {
      match id_header_value.to_str() {
        Ok(id_string) => match id_string.parse::<uuid::Uuid>() {
          Ok(client_id) => {
            match get_tenant_by_client_id(&router_state.pool, &client_id.to_string()).await {
              Ok(Some(tenant)) => Ok(TenantId(tenant)),
              Ok(None) => {
                Err(InternalError::bad_request().with_error(TENENT_ID_HEADER, INVALID_ERROR))
              }
              Err(e) => {
                log::error!("invalid tenant id: {}", e);
                Err(InternalError::bad_request().with_error(TENENT_ID_HEADER, INVALID_ERROR))
              }
            }
          }
          Err(e) => {
            log::error!("invalid tenant id: {}", e);
            Err(InternalError::bad_request().with_error(TENENT_ID_HEADER, INVALID_ERROR))
          }
        },
        Err(e) => {
          log::error!("invalid tenant id: {}", e);
          Err(InternalError::bad_request().with_error(TENENT_ID_HEADER, PARSE_ERROR))
        }
      }
    } else {
      Err(InternalError::bad_request().with_error(TENENT_ID_HEADER, REQUIRED_ERROR))
    }
  }
}
