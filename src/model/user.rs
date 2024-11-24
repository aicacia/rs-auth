use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::repository::user::UserRow;

#[derive(Serialize, ToSchema)]
pub struct User {
  pub id: i64,
  pub username: String,
  pub active: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<UserRow> for User {
  fn from(row: UserRow) -> Self {
    Self {
      id: row.id,
      username: row.username,
      active: row.active != 0,
      created_at: DateTime::<Utc>::from_timestamp(row.created_at, 0).unwrap_or_default(),
      updated_at: DateTime::<Utc>::from_timestamp(row.updated_at, 0).unwrap_or_default(),
    }
  }
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct CreateUser {
  pub username: String,
  pub active: bool,
}
