use config::ConfigError;
use serde::Deserialize;
use std::{
  net::{IpAddr, Ipv4Addr},
  sync::{atomic::Ordering, Arc},
};

use super::atomic_value::AtomicValue;

lazy_static! {
  static ref CONFIG: AtomicValue<Config> = AtomicValue::new(Config::default());
}

pub async fn init_config() -> Result<Arc<Config>, ConfigError> {
  CONFIG.set(Config::new().await?, Ordering::Relaxed);
  Ok(get_config())
}

pub fn get_config() -> Arc<Config> {
  CONFIG.get(Ordering::Relaxed)
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
  pub address: IpAddr,
  pub port: u16,
  pub url: String,
}

impl Default for ServerConfig {
  fn default() -> Self {
    Self {
      address: IpAddr::from(Ipv4Addr::UNSPECIFIED),
      port: 3000,
      url: "http://localhost:3000".to_owned(),
    }
  }
}

#[derive(Debug, Deserialize, Default)]
pub struct DatabaseConfig {
  pub url: String,
  pub min_connections: u32,
  pub max_connections: u32,
  pub connect_timeout: u64,
  pub acquire_timeout: u64,
  pub idle_timeout: u64,
  pub max_lifetime: u64,
  pub journal_mode: String,
  pub synchronize: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct PasswordConfig {
  pub salt_length: u8,
  pub hash_length: u8,
  pub memory_mib: u8,
  pub iterations: u8,
  pub parallelism: u8,
}

#[derive(Debug, Deserialize, Default)]
pub struct RegisterConfig {
  pub enabled: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct UserConfig {
  pub register: RegisterConfig,
  pub allow_passwords: bool,
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
  pub server: ServerConfig,
  pub database: DatabaseConfig,
  pub password: PasswordConfig,
  pub user: UserConfig,
  pub log_level: String,
}

impl Config {
  pub async fn new() -> Result<Self, ConfigError> {
    let config_builder = config::Config::builder()
      // Server Defaults
      .set_default("server.address", "0.0.0.0")?
      .set_default("server.port", 3000)?
      .set_default("server.url", "http://localhost:3000")?
      // Database Defaults
      .set_default(
        "database.url",
        std::env::var("DATABASE_URL").unwrap_or_default(),
      )?
      .set_default("database.min_connections", 1)?
      .set_default("database.max_connections", 100)?
      .set_default("database.connect_timeout", 3)?
      .set_default("database.acquire_timeout", 3)?
      .set_default("database.idle_timeout", 5)?
      .set_default("database.max_lifetime", 300)?
      .set_default("database.journal_mode", "wal")?
      .set_default("database.synchronize", true)?
      // Password Defaults
      .set_default("password.salt_length", 16)?
      .set_default("password.hash_length", 32)?
      .set_default("password.memory_mib", 19)?
      .set_default("password.iterations", 2)?
      .set_default("password.parallelism", 1)?
      // User Defaults
      .set_default("user.register.enabled", true)?
      .set_default("user.allow_passwords", true)?
      // Defaults
      .set_default("log_level", "debug")?
      .add_source(config::File::with_name("./config.json"))
      .add_source(config::Environment::with_prefix("APP"))
      .build()?;

    let config = config_builder.try_deserialize()?;
    Ok(config)
  }
}
