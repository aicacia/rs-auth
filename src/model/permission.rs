use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Role {
  pub id: i32,
  pub name: String,
  pub uri: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Permission {
  pub id: i32,
  pub key: String,
  #[sqlx(rename = "type")]
  pub kind: serde_json::Value,
  pub default: serde_json::Value,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct RolePermission {
  pub role_id: i32,
  pub permission_id: i32,
  pub value: serde_json::Value,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
