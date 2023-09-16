use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Application {
  pub id: i32,
  pub name: String,
  pub uri: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
