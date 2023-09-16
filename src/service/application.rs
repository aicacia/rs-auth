use sqlx::{Pool, Postgres};

pub async fn get_application_config(
  pool: &Pool<Postgres>,
  application_id: i32,
  key: &str,
) -> serde_json::Value {
  sqlx::query!(
    "SELECT value FROM application_configs WHERE application_id = $1 AND name = $2 LIMIT 1;",
    application_id,
    key
  )
  .fetch_optional(pool)
  .await
  .map_or(serde_json::Value::Null, |v| {
    v.map_or(serde_json::Value::Null, |r| r.value)
  })
}

pub async fn set_application_config(
  pool: &Pool<Postgres>,
  application_id: i32,
  key: &str,
  value: serde_json::Value,
) -> serde_json::Value {
  sqlx::query!(
    "INSERT INTO application_configs
    (application_id, name, value) VALUES ($1, $2, $3)
    ON CONFLICT (application_id, name)
    DO UPDATE SET value = $3
    RETURNING value;",
    application_id,
    key,
    value
  )
  .fetch_optional(pool)
  .await
  .map_or(serde_json::Value::Null, |v| {
    v.map_or(serde_json::Value::Null, |r| r.value)
  })
}
