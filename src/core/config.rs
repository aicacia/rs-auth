use config::ConfigError;
use serde::Deserialize;
use std::{
  net::IpAddr,
  sync::{Arc, atomic::Ordering},
};

use super::atomic_value::AtomicValue;

static CONFIG: AtomicValue<Arc<Config>> = AtomicValue::empty();

pub async fn init_config() -> Result<Arc<Config>, ConfigError> {
  let config = Arc::new(Config::new().await?);
  CONFIG.set(config.clone(), Ordering::SeqCst);
  Ok(config)
}

pub fn get_config() -> Arc<Config> {
  assert!(!CONFIG.is_empty(), "Config not initialized");
  CONFIG.get(Ordering::Relaxed)
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
  pub address: IpAddr,
  pub port: u16,
  pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
  pub url: String,
  pub min_connections: u32,
  pub max_connections: u32,
  pub connect_timeout: u64,
  pub acquire_timeout: u64,
  pub idle_timeout: u64,
  pub max_lifetime: u64,
}

#[derive(Debug, Deserialize)]
pub struct PasswordConfig {
  pub salt_length: usize,
  pub hash_length: u32,
  pub memory_mib: u32,
  pub iterations: u32,
  pub parallelism: u32,
}

#[derive(Debug, Deserialize)]
pub struct RegisterConfig {
  pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct UserConfig {
  pub register: RegisterConfig,
  pub allow_passwords: bool,
}

#[derive(Debug, Deserialize)]
pub struct OAuth2Config {
  pub name: String,
  pub enabled: bool,
  pub client_id: String,
  pub client_secret: String,
  pub auth_url: String,
  pub token_url: String,
  pub scopes: Vec<String>,
  pub redirect_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OAuth2 {
  pub redirect_url: String,
  pub code_timeout_in_seconds: u64,
  pub google: OAuth2Config,
}

#[derive(Debug, Deserialize)]
pub struct Config {
  pub server: ServerConfig,
  pub database: DatabaseConfig,
  pub password: PasswordConfig,
  pub user: UserConfig,
  pub oauth2: OAuth2,
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
      // Password Defaults
      .set_default("password.salt_length", 16)?
      .set_default("password.hash_length", 32)?
      .set_default("password.memory_mib", 19)?
      .set_default("password.iterations", 2)?
      .set_default("password.parallelism", 1)?
      // User Defaults
      .set_default("user.register.enabled", true)?
      .set_default("user.allow_passwords", true)?
      // OAuth2 Defaults
      .set_default("oauth2.code_timeout_in_seconds", 60 * 5)?
      .set_default("oauth2.google.enabled", false)?
      .set_default(
        "oauth2.google.auth_url",
        "https://accounts.google.com/o/oauth2/v2/auth",
      )?
      .set_default(
        "oauth2.google.token_url",
        "https://www.googleapis.com/oauth2/v3/token",
      )?
      .set_default("oauth2.google.scopes", vec![
        "https://www.googleapis.com/auth/userinfo.email",
        "https://www.googleapis.com/auth/userinfo.profile",
      ])?
      // Defaults
      .set_default("log_level", "debug")?
      .add_source(config::File::with_name("./config.json"))
      .add_source(config::Environment::with_prefix("APP"))
      .build()?;

    let config = config_builder.try_deserialize()?;
    Ok(config)
  }
}
