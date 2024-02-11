use anyhow::Result;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::{net::IpAddr, sync::Arc};

use crate::service::config::get_configs_map;

use super::{atomic_value::AtomicValue, db::start_listening};

lazy_static! {
  static ref CONFIG: AtomicValue<Config> = AtomicValue::new(Config::default());
}

pub async fn init_config(pool: &Pool<Postgres>) -> Result<()> {
  CONFIG.set(Config::new(pool).await?);
  Ok(())
}

pub fn get_config() -> Arc<Config> {
  CONFIG.get()
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct ServerConfig {
  pub address: Option<IpAddr>,
  pub port: u16,
  pub uri: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Config {
  pub server: ServerConfig,
  pub log_level: String,
  pub admin_application_id: i32,
}

impl Config {
  pub async fn new(pool: &Pool<Postgres>) -> Result<Self> {
    let config_builder = config::Config::builder()
      .add_source(RawSource::new(get_configs_map(pool).await?))
      // App
      .set_default("log_level", "info")?
      // build
      .build()?;

    let config = config_builder.try_deserialize()?;
    Ok(config)
  }
}

#[derive(Clone, Debug)]
pub struct RawSource {
  map: config::Map<String, config::Value>,
}

impl RawSource {
  pub fn new(map: config::Map<String, config::Value>) -> Self {
    Self { map }
  }
}

impl config::Source for RawSource {
  fn clone_into_box(&self) -> Box<dyn config::Source + Send + Sync> {
    Box::new(self.clone())
  }

  fn collect(&self) -> Result<config::Map<String, config::Value>, config::ConfigError> {
    Ok(self.map.clone())
  }
}

pub async fn config_listener(pool: Pool<Postgres>) -> Result<()> {
  start_listening(
    pool.clone(),
    vec!["config_channel"],
    move |payload: Payload, pool| async move {
      log::info!("Config update: {:?}", payload);
      CONFIG.set(Config::new(&pool).await?);
      Ok(())
    },
  )
  .await?;
  Ok(())
}

#[derive(Deserialize, Debug)]
pub enum ActionType {
  INSERT,
  UPDATE,
  DELETE,
}

#[derive(Deserialize, Debug)]
pub struct Payload {
  pub table: String,
  pub action_type: ActionType,
  pub key: String,
  pub value: serde_json::Value,
}
