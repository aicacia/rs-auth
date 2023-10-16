use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use chrono::{DateTime, Utc};
use futures::future::{err, ok};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::error::Errors;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct ApplicationRow {
  pub id: i32,
  pub name: String,
  pub uri: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl FromRequest for ApplicationRow {
  type Error = actix_web::Error;
  type Future = futures::future::Ready<Result<Self, Self::Error>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    match req.extensions().get::<ApplicationRow>() {
      Some(application) => ok(application.clone()),
      None => {
        let mut error = Errors::new();
        error.error("application", "invalid");
        err(actix_web::error::ErrorUnauthorized(error))
      }
    }
  }
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Application {
  pub id: i32,
  pub name: String,
  pub uri: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<ApplicationRow> for Application {
  fn from(application: ApplicationRow) -> Self {
    Self {
      id: application.id,
      name: application.name,
      uri: application.uri,
      created_at: application.created_at,
      updated_at: application.updated_at,
    }
  }
}
