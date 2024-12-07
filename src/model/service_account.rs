use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

use crate::repository::service_account::ServiceAccountRow;

#[derive(Serialize, ToSchema)]
pub struct ServiceAccount {
  pub id: i64,
  pub client_id: uuid::Uuid,
  pub name: String,
  pub active: bool,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

impl From<ServiceAccountRow> for ServiceAccount {
  fn from(row: ServiceAccountRow) -> Self {
    let active = row.is_active();
    Self {
      id: row.id,
      client_id: uuid::Uuid::parse_str(&row.client_id).unwrap_or_default(),
      name: row.name,
      active,
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
    }
  }
}
