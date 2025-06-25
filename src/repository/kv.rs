use chrono::Duration;
use serde::{Serialize, de::DeserializeOwned};

#[derive(Clone, sqlx::FromRow)]
pub struct KVRow {
  pub key: String,
  pub value: String,
  pub expries_at: Option<i64>,
  pub updated_at: i64,
  pub created_at: i64,
}

pub async fn kv_get(pool: &sqlx::AnyPool, key: String) -> sqlx::Result<Option<KVRow>> {
  sqlx::query_as(r#"SELECT * FROM key_values WHERE "key" = $1 AND ("updated_at" IS NULL OR "updated_at" <= $2);"#)
    .bind(key)
    .bind(chrono::Utc::now().timestamp())
    .fetch_optional(pool)
    .await
}

pub async fn kv_upsert(
  pool: &sqlx::AnyPool,
  key: String,
  value: String,
  expires_at: Option<i64>,
) -> sqlx::Result<KVRow> {
  sqlx::query_as(
    r#"INSERT INTO key_values ("key", "value", "expries_at")
        VALUES ($1, $2, $3)
        ON CONFLICT ("key")
        DO UPDATE SET "value" = $2, "expries_at" = $3, "updated_at" = $5
        RETURNING *;"#,
  )
  .bind(key)
  .bind(value)
  .bind(expires_at)
  .bind(chrono::Utc::now().timestamp())
  .fetch_one(pool)
  .await
}

pub async fn kv_delete(pool: &sqlx::AnyPool, key: String) -> sqlx::Result<Option<KVRow>> {
  sqlx::query_as(r#"DELETE FROM key_values WHERE "key" = $1 RETURNING *;"#)
    .bind(key)
    .fetch_optional(pool)
    .await
    .map(|row_optional: Option<KVRow>| {
      row_optional.and_then(|row| {
        if let Some(expires_at) = row.expries_at {
          if expires_at < chrono::Utc::now().timestamp() {
            return None;
          }
        }
        Some(row)
      })
    })
}

pub async fn get<S, T>(pool: &sqlx::AnyPool, key: S) -> Option<T>
where
  S: Into<String>,
  T: DeserializeOwned,
{
  match kv_get(pool, key.into()).await {
    Ok(Some(row)) => match serde_json::from_str(&row.value) {
      Ok(value) => Some(value),
      _ => None,
    },
    _ => None,
  }
}

pub async fn set<S, T>(pool: &sqlx::AnyPool, key: S, value: &T, expires: Option<Duration>) -> bool
where
  S: Into<String>,
  T: Serialize,
{
  match serde_json::to_string(value) {
    Ok(value) => {
      match kv_upsert(
        pool,
        key.into(),
        value,
        expires
          .and_then(|e| chrono::Utc::now().checked_add_signed(e))
          .map(|d| d.timestamp()),
      )
      .await
      {
        Ok(_) => true,
        _ => false,
      }
    }
    _ => false,
  }
}

pub async fn delete<S, T>(pool: &sqlx::AnyPool, key: S) -> Option<T>
where
  S: Into<String>,
  T: DeserializeOwned,
{
  match kv_delete(pool, key.into()).await {
    Ok(Some(row)) => match serde_json::from_str(&row.value) {
      Ok(value) => Some(value),
      _ => None,
    },
    _ => None,
  }
}
