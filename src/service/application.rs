use anyhow::Result;
use sqlx::{Pool, Postgres};

use crate::model::application::ApplicationRow;

pub async fn get_application_by_id(
  pool: &Pool<Postgres>,
  application_id: i32,
) -> Result<ApplicationRow> {
  let application = sqlx::query_as!(
    ApplicationRow,
    r#"SELECT
      a.id, a.name, a.uri, a.created_at, a.updated_at
    FROM applications a
    WHERE a.id = $1
    LIMIT 1;"#,
    application_id
  )
  .fetch_one(pool)
  .await?;
  Ok(application)
}

pub async fn get_applications(pool: &Pool<Postgres>) -> Result<Vec<ApplicationRow>> {
  let applications = sqlx::query_as!(
    ApplicationRow,
    r#"SELECT
      a.id, a.name, a.uri, a.created_at, a.updated_at
    FROM applications a
    LIMIT 1;"#
  )
  .fetch_all(pool)
  .await?;
  Ok(applications)
}

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

pub async fn get_application_jwt_expires_in_seconds(
  pool: &Pool<Postgres>,
  application_id: i32,
) -> usize {
  get_application_config(pool, application_id, "jwt.expires_in_seconds")
    .await
    .as_u64()
    .unwrap_or(86400) as usize
}

pub async fn get_application_jwt_secret(pool: &Pool<Postgres>, application_id: i32) -> String {
  get_application_config(pool, application_id, "jwt.secret")
    .await
    .as_str()
    .unwrap_or_default()
    .to_owned()
}

pub async fn get_application_uri(pool: &Pool<Postgres>, application_id: i32) -> String {
  get_application_config(pool, application_id, "uri")
    .await
    .as_str()
    .unwrap_or_default()
    .to_owned()
}
