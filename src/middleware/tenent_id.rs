use std::collections::HashMap;

use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use serde_json::json;

use crate::{
  core::{
    error::{Errors, PARSE_ERROR, REQUIRED_ERROR},
    openapi::TENENT_ID_HEADER,
  },
  repository::tenent::{get_tenent_by_client_id, TenentRow},
  router::RouterState,
};

pub struct TenentId(pub TenentRow);

impl<S> FromRequestParts<S> for TenentId
where
  RouterState: FromRef<S>,
  S: Send + Sync,
{
  type Rejection = Errors;

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let router_state = RouterState::from_ref(state);

    if let Some(id_header_value) = parts.headers.get(TENENT_ID_HEADER) {
      match id_header_value.to_str() {
        Ok(id_string) => match id_string.parse::<uuid::Uuid>() {
          Ok(client_id) => match get_tenent_by_client_id(&router_state.pool, &client_id).await {
            Ok(Some(tenent)) => Ok(TenentId(tenent)),
            Ok(None) => Err(Errors::bad_request().with_error(
              TENENT_ID_HEADER,
              (
                PARSE_ERROR,
                HashMap::from([("in".to_owned(), json!("header"))]),
              ),
            )),
            Err(e) => {
              log::error!("invalid tenent id: {}", e);
              Err(Errors::bad_request().with_error(
                TENENT_ID_HEADER,
                (
                  PARSE_ERROR,
                  HashMap::from([("in".to_owned(), json!("header"))]),
                ),
              ))
            }
          },
          Err(e) => {
            log::error!("invalid tenent id: {}", e);
            Err(Errors::bad_request().with_error(
              TENENT_ID_HEADER,
              (
                PARSE_ERROR,
                HashMap::from([("in".to_owned(), json!("header"))]),
              ),
            ))
          }
        },
        Err(e) => {
          log::error!("invalid tenent id: {}", e);
          Err(Errors::bad_request().with_error(
            TENENT_ID_HEADER,
            (
              REQUIRED_ERROR,
              HashMap::from([("in".to_owned(), json!("header"))]),
            ),
          ))
        }
      }
    } else {
      Err(Errors::bad_request().with_error(
        TENENT_ID_HEADER,
        (
          REQUIRED_ERROR,
          HashMap::from([("in".to_owned(), json!("header"))]),
        ),
      ))
    }
  }
}
