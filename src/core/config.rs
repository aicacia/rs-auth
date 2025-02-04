use config::ConfigError;
use serde::Deserialize;
use std::net::IpAddr;

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
pub struct P2PConfig {
  pub enabled: bool,
  pub tenant_client_id: uuid::Uuid,
  pub ws_uri: String,
  pub api_uri: String,
  pub id: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordConfig {
  pub salt_length: usize,
  pub hash_length: u32,
  pub memory_mib: u32,
  pub iterations: u32,
  pub parallelism: u32,
  pub history: u8,
  pub expire_days: u8,
}

#[derive(Debug, Deserialize)]
pub struct UserConfig {
  pub register_enabled: bool,
  pub allow_passwords: bool,
  pub allow_mfa_totp: bool,
  pub allow_mfa_text: bool,
  pub allow_mfa_email: bool,
}

#[derive(Debug, Deserialize)]
pub struct OAuth2 {
  pub register_enabled: bool,
  pub code_timeout_in_seconds: u64,
}

#[derive(Debug, Deserialize)]
pub struct Config {
  pub server: ServerConfig,
  pub database: DatabaseConfig,
  pub p2p: P2PConfig,
  pub password: PasswordConfig,
  pub user: UserConfig,
  pub oauth2: OAuth2,
  pub log_level: String,
}

impl Config {
  pub async fn new(config_path: &str) -> Result<Self, ConfigError> {
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
      .set_default("database.connect_timeout", 10)?
      .set_default("database.acquire_timeout", 10)?
      .set_default("database.idle_timeout", 10)?
      .set_default("database.max_lifetime", 300)?
      // P2P
      .set_default("p2p.enabled", false)?
      .set_default(
        "p2p.tenant_client_id",
        "6fcf0235-cb11-4160-9df8-b9114f8dcdae",
      )?
      .set_default("p2p.ws_uri", "wss://p2p.aicacia.com".to_owned())?
      .set_default("p2p.api_uri", "https://p2p.aicacia.com".to_owned())?
      .set_default("p2p.id", uuid::Uuid::new_v4().to_string())?
      .set_default("p2p.password", uuid::Uuid::new_v4().to_string())?
      // Password Defaults
      .set_default("password.salt_length", 16)?
      .set_default("password.hash_length", 32)?
      .set_default("password.memory_mib", 19)?
      .set_default("password.iterations", 2)?
      .set_default("password.parallelism", 1)?
      .set_default("password.history", 24)?
      .set_default("password.expire_days", 60)?
      // User Defaults
      .set_default("user.register_enabled", false)?
      .set_default("user.allow_passwords", true)?
      .set_default("user.allow_mfa_totp", true)?
      .set_default("user.allow_mfa_email", true)?
      .set_default("user.allow_mfa_text", true)?
      // OAuth2 Defaults
      .set_default("oauth2.register_enabled", false)?
      .set_default("oauth2.code_timeout_in_seconds", 60 * 5)?
      // Defaults
      .set_default("log_level", "debug")?
      .add_source(config::File::with_name(config_path))
      .add_source(config::Environment::with_prefix("APP"))
      .build()?;

    let config = config_builder.try_deserialize()?;
    Ok(config)
  }
}
