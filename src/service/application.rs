use anyhow::Result;
use futures::try_join;
use serde_json::json;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
  core::jwt::gen_jwt_secret,
  model::application::{ApplicationConfigRow, ApplicationRow},
};

pub async fn get_applications(
  pool: &Pool<Postgres>,
  page: i64,
  page_size: i64,
) -> Result<Vec<ApplicationRow>> {
  let applications = sqlx::query_as!(
    ApplicationRow,
    r#"SELECT
      a.id, a.name, a.uri, a.secret, a.created_at, a.updated_at
    FROM applications a
    ORDER BY a.updated_at DESC
    LIMIT $1 OFFSET $2;"#,
    page_size,
    page * page_size
  )
  .fetch_all(pool)
  .await?;
  Ok(applications)
}

pub async fn get_application_by_id(
  pool: &Pool<Postgres>,
  application_id: Uuid,
) -> Result<Option<ApplicationRow>> {
  let application = sqlx::query_as!(
    ApplicationRow,
    r#"SELECT
      a.id, a.name, a.uri, a.secret, a.created_at, a.updated_at
    FROM applications a
    WHERE a.id = $1
    LIMIT 1;"#,
    application_id
  )
  .fetch_optional(pool)
  .await?;
  Ok(application)
}

pub async fn create_application(
  pool: &Pool<Postgres>,
  name: &String,
  uri: &String,
) -> Result<ApplicationRow> {
  let application = sqlx::query_as!(
    ApplicationRow,
    r#"INSERT INTO applications (name, uri)
    VALUES ($1, $2)
    RETURNING id, name, uri, secret, created_at, updated_at;"#,
    name,
    uri
  )
  .fetch_one(pool)
  .await?;

  try_join!(
    async {
      create_application_config(
        &pool,
        application.id,
        "jwt.secret",
        &json!(gen_jwt_secret()),
      )
      .await
    },
    async {
      create_application_config(
        &pool,
        application.id,
        "jwt.expires_in_seconds",
        &json!(86400),
      )
      .await
    }
  )?;

  Ok(application)
}

pub async fn update_application(
  pool: &Pool<Postgres>,
  application_id: Uuid,
  name: Option<&String>,
  uri: Option<&String>,
) -> Result<Option<ApplicationRow>> {
  let application = sqlx::query_as!(
    ApplicationRow,
    r#"UPDATE applications
    SET name = COALESCE($1, name),
        uri = COALESCE($2, uri)
    WHERE id = $3
    RETURNING id, name, uri, secret, created_at, updated_at;"#,
    name,
    uri,
    application_id
  )
  .fetch_optional(pool)
  .await?;
  Ok(application)
}

pub async fn delete_application(
  pool: &Pool<Postgres>,
  application_id: Uuid,
) -> Result<Option<ApplicationRow>> {
  let application = sqlx::query_as!(
    ApplicationRow,
    r#"DELETE FROM applications
    WHERE id = $1
    RETURNING id, name, uri, secret, created_at, updated_at;"#,
    application_id
  )
  .fetch_optional(pool)
  .await?;
  Ok(application)
}

pub async fn reset_application_secret(
  pool: &Pool<Postgres>,
  application_id: Uuid,
) -> Result<Option<ApplicationRow>> {
  Ok(
    sqlx::query_as!(
      ApplicationRow,
      r#"UPDATE applications
      SET secret=encode(gen_random_bytes(64), 'base64')
      WHERE id=$1
      RETURNING id, name, uri, secret, created_at, updated_at;"#,
      application_id
    )
    .fetch_optional(pool)
    .await?,
  )
}

pub async fn get_application_configs(
  pool: &Pool<Postgres>,
  application_id: Uuid,
) -> Result<Vec<ApplicationConfigRow>> {
  let application_configs = sqlx::query_as!(
    ApplicationConfigRow,
    "SELECT ac.application_id, ac.key, ac.value, ac.created_at, ac.updated_at FROM application_configs ac WHERE ac.application_id = $1;",
    application_id
  )
  .fetch_all(pool)
  .await?;
  Ok(application_configs)
}

pub async fn get_application_config(
  pool: &Pool<Postgres>,
  application_id: Uuid,
  key: &str,
) -> serde_json::Value {
  sqlx::query!(
    "SELECT value FROM application_configs WHERE application_id = $1 AND key = $2 LIMIT 1;",
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
  application_id: Uuid,
  key: &str,
  value: &serde_json::Value,
) -> Result<()> {
  if sqlx::query!(
    "SELECT * FROM application_configs WHERE application_id=$1 AND key=$2 LIMIT 1;",
    application_id,
    key
  )
  .fetch_optional(pool)
  .await?
  .is_some()
  {
    update_application_config(pool, application_id, key, value).await
  } else {
    create_application_config(pool, application_id, key, value).await
  }
}

pub async fn update_application_config(
  pool: &Pool<Postgres>,
  application_id: Uuid,
  key: &str,
  value: &serde_json::Value,
) -> Result<()> {
  sqlx::query!(
    "UPDATE application_configs SET value = $3 WHERE application_id=$1 AND key=$2;",
    application_id,
    key,
    value
  )
  .fetch_optional(pool)
  .await?;
  Ok(())
}

pub async fn create_application_config(
  pool: &Pool<Postgres>,
  application_id: Uuid,
  key: &str,
  value: &serde_json::Value,
) -> Result<()> {
  sqlx::query_as!(
    ApplicationConfigRow,
    "INSERT INTO application_configs (application_id, key, value) VALUES ($1, $2, $3);",
    application_id,
    key,
    value
  )
  .execute(pool)
  .await?;
  Ok(())
}

pub async fn get_application_jwt_expires_in_seconds(
  pool: &Pool<Postgres>,
  application_id: Uuid,
) -> usize {
  get_application_config(pool, application_id, "jwt.expires_in_seconds")
    .await
    .as_u64()
    .unwrap_or(86400) as usize
}

pub async fn get_application_jwt_secret(pool: &Pool<Postgres>, application_id: Uuid) -> String {
  get_application_config(pool, application_id, "jwt.secret")
    .await
    .as_str()
    .unwrap_or_default()
    .to_owned()
}

pub async fn get_application_uri(pool: &Pool<Postgres>, application_id: Uuid) -> String {
  get_application_config(pool, application_id, "uri")
    .await
    .as_str()
    .unwrap_or_default()
    .to_owned()
}
