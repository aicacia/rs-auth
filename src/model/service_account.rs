use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::repository::service_account::ServiceAccountRow;

#[derive(Serialize, ToSchema)]
pub struct ServiceAccount {
  pub id: i64,
  pub client_id: uuid::Uuid,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub client_secret: Option<uuid::Uuid>,
  pub name: String,
  pub active: bool,
  pub admin: bool,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl From<ServiceAccountRow> for ServiceAccount {
  fn from(row: ServiceAccountRow) -> Self {
    let active = row.is_active();
    let admin = row.is_admin();
    Self {
      id: row.id,
      client_id: uuid::Uuid::parse_str(&row.client_id).unwrap_or_default(),
      name: row.name,
      client_secret: None,
      active,
      admin,
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
    }
  }
}

#[derive(Serialize, ToSchema)]
pub struct ServiceAccountPagination {
  pub has_more: bool,
  pub items: Vec<ServiceAccount>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateServiceAccount {
  pub name: String,
  pub client_id: Option<uuid::Uuid>,
  pub client_secret: Option<uuid::Uuid>,
  pub admin: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateServiceAccount {
  pub name: Option<String>,
  pub client_id: Option<uuid::Uuid>,
  pub client_secret: Option<uuid::Uuid>,
  pub admin: Option<bool>,
  pub active: Option<bool>,
}
