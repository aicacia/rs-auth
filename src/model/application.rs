use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use chrono::{DateTime, Utc};
use futures::future::{err, ok};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use super::auth::validate_no_whitespace;
use super::error::Errors;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct ApplicationRow {
  pub id: i32,
  pub description: String,
  pub uri: String,
  pub secret: String,
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
  pub description: String,
  pub uri: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<ApplicationRow> for Application {
  fn from(application: ApplicationRow) -> Self {
    Self {
      id: application.id,
      description: application.description,
      uri: application.uri,
      created_at: application.created_at,
      updated_at: application.updated_at,
    }
  }
}

#[derive(Deserialize, Validate, IntoParams)]
pub struct PaginationApplicationQuery {
  pub page: Option<i64>,
  pub page_size: Option<i64>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PaginationApplication {
  pub has_more: bool,
  pub data: Vec<Application>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ApplicationWithSecret {
  pub id: i32,
  pub description: String,
  pub uri: String,
  pub secret: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<ApplicationRow> for ApplicationWithSecret {
  fn from(application: ApplicationRow) -> Self {
    Self {
      id: application.id,
      description: application.description,
      uri: application.uri,
      secret: application.secret,
      created_at: application.created_at,
      updated_at: application.updated_at,
    }
  }
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct ApplicationPermissionRow {
  pub id: i32,
  pub application_id: i32,
  pub description: String,
  pub uri: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ApplicationPermission {
  pub id: i32,
  pub application_id: i32,
  pub description: String,
  pub uri: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<ApplicationPermissionRow> for ApplicationPermission {
  fn from(application_permission: ApplicationPermissionRow) -> Self {
    Self {
      id: application_permission.id,
      application_id: application_permission.application_id,
      description: application_permission.description,
      uri: application_permission.uri,
      created_at: application_permission.created_at,
      updated_at: application_permission.updated_at,
    }
  }
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct ApplicationConfigRow {
  pub application_id: i32,
  pub key: String,
  pub value: Value,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ApplicationConfig {
  pub key: String,
  pub value: Value,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<ApplicationConfigRow> for ApplicationConfig {
  fn from(application_config: ApplicationConfigRow) -> Self {
    Self {
      key: application_config.key,
      value: application_config.value,
      created_at: application_config.created_at,
      updated_at: application_config.updated_at,
    }
  }
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct CreateApplicationRequest {
  pub description: String,
  #[validate(length(min = 1), custom = "validate_no_whitespace")]
  pub uri: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct UpdateApplicationRequest {
  pub description: Option<String>,
  #[validate(length(min = 1), custom = "validate_no_whitespace")]
  pub uri: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Validate)]
pub struct UpdateApplicationConfigRequest {
  pub key: String,
  pub value: Value,
}
