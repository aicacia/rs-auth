use dashmap::DashMap;
use sqlx::{Pool, Postgres};
use tokio::time::Instant;

struct TTLItem {
  value: serde_json::Value,
  ttl: Instant,
}

lazy_static! {
  static ref SETTINGS_RAW: DashMap<String, TTLItem> = DashMap::new();
}

pub async fn get_setting(pool: &Pool<Postgres>, key: &str) -> serde_json::Value {
  if let Some(value) = SETTINGS_RAW.get(key) {
    if value.ttl.elapsed().as_secs() < 60 {
      return value.value.clone();
    }
  }
  let value = sqlx::query!("SELECT value FROM config WHERE name = $1 LIMIT 1;", key)
    .fetch_optional(pool)
    .await
    .map_or(Some(serde_json::Value::Null), |v| {
      v.map_or(Some(serde_json::Value::Null), |r| r.value)
    })
    .unwrap_or(serde_json::Value::Null);

  SETTINGS_RAW.insert(
    key.into(),
    TTLItem {
      value: value.clone(),
      ttl: Instant::now(),
    },
  );

  value
}
