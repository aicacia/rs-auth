use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn get_config_map(pool: &Pool<Postgres>) -> Result<config::Map<String, config::Value>> {
  let config = sqlx::query!("SELECT name, value FROM config;")
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| (r.name, json_value_to_config_value(r.value)))
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

pub async fn get_config(pool: &Pool<Postgres>, key: &str) -> serde_json::Value {
  sqlx::query!("SELECT value FROM config WHERE name = $1 LIMIT 1;", key)
    .fetch_optional(pool)
    .await
    .map_or(serde_json::Value::Null, |v| {
      v.map_or(serde_json::Value::Null, |r| r.value)
    })
}
