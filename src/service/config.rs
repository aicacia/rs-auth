use std::collections::HashMap;

use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn get_config(pool: &Pool<Postgres>, key: &str) -> serde_json::Value {
  get_config_or(pool, key, serde_json::Value::Null).await
}

pub async fn get_config_or(
  pool: &Pool<Postgres>,
  key: &str,
  default: serde_json::Value,
) -> serde_json::Value {
  get_config_or_else(pool, key, || default).await
}

pub async fn get_config_or_else<F>(
  pool: &Pool<Postgres>,
  key: &str,
  default_fn: F,
) -> serde_json::Value
where
  F: FnOnce() -> serde_json::Value,
{
  sqlx::query!("SELECT value FROM config WHERE key = $1 LIMIT 1;", key)
    .fetch_optional(pool)
    .await
    .ok()
    .map(|v| v.map(|r| r.value))
    .flatten()
    .unwrap_or_else(default_fn)
}

pub async fn get_configs(pool: &Pool<Postgres>) -> Result<HashMap<String, serde_json::Value>> {
  let config = sqlx::query!("SELECT key, value FROM config;")
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| (r.key, r.value))
    .collect::<HashMap<String, serde_json::Value>>();
  Ok(config)
}

pub async fn get_configs_map(pool: &Pool<Postgres>) -> Result<config::Map<String, config::Value>> {
  let config = get_configs(pool)
    .await?
    .into_iter()
    .map(|(k, v)| (k, json_value_to_config_value(v)))
    .collect::<config::Map<String, config::Value>>();
  Ok(config)
}

fn json_value_to_config_value(value: serde_json::Value) -> config::Value {
  match value {
    serde_json::Value::Null => config::Value::new(None, config::ValueKind::Nil),
    serde_json::Value::Bool(b) => config::Value::new(None, config::ValueKind::Boolean(b)),
    serde_json::Value::Number(n) => config::Value::new(None, {
      if let Some(i) = n.as_i64() {
        config::ValueKind::I64(i)
      } else if let Some(u) = n.as_u64() {
        config::ValueKind::U64(u)
      } else if let Some(f) = n.as_f64() {
        config::ValueKind::Float(f)
      } else {
        config::ValueKind::Float(f64::NAN)
      }
    }),
    serde_json::Value::String(s) => config::Value::new(None, config::ValueKind::String(s)),
    serde_json::Value::Array(a) => config::Value::new(
      None,
      config::ValueKind::Array(
        a.into_iter()
          .map(json_value_to_config_value)
          .collect::<Vec<_>>(),
      ),
    ),
    serde_json::Value::Object(m) => config::Value::new(
      None,
      config::ValueKind::Table(
        m.into_iter()
          .map(|(k, b)| (k, json_value_to_config_value(b)))
          .collect::<config::Map<_, _>>(),
      ),
    ),
  }
}
