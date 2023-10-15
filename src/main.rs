use std::net::{IpAddr, Ipv4Addr};

use actix_web::HttpServer;
use anyhow::Result;
use futures::join;
use serde::Deserialize;

use auth::{
  app::create_app,
  core::{
    config::{get_config, init_config},
    db::{create_pool, start_listening},
  },
};

#[actix_web::main]
async fn main() -> Result<()> {
  dotenv::dotenv()?;

  let pool = create_pool().await?;
  init_config(&pool).await?;

  env_logger::init_from_env(env_logger::Env::new().default_filter_or(&get_config().log_level));

  let host = std::env::var("HOST")
    .unwrap_or_default()
    .parse::<IpAddr>()
    .ok()
    .unwrap_or(
      get_config()
        .server
        .address
        .unwrap_or(IpAddr::from(Ipv4Addr::UNSPECIFIED)),
    );

  let port = std::env::var("PORT")
    .unwrap_or_default()
    .parse::<u16>()
    .ok()
    .unwrap_or(get_config().server.port);

  let http_pool = pool.clone();
  let server = HttpServer::new(move || create_app(&http_pool))
    .bind((host, port))?
    .run();

  let server_handle = tokio::spawn(server);
  let listener_handle = tokio::spawn(start_listening(
    pool.clone(),
    vec!["config_channel"],
    |payload: Payload| async move {
      log::info!("Received config update: {:?}", payload);
      Ok(())
    },
  ));

  let result: Result<()> = match join!(listener_handle, server_handle) {
    (Ok(_), Ok(_)) => Ok(()),
    (Err(e), Ok(_)) => Err(e.into()),
    (Ok(_), Err(e)) => Err(e.into()),
    (Err(e), Err(e2)) => Err(anyhow::anyhow!("{:?} {:?}", e, e2)),
  };

  pool.close().await;

  result
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
  pub name: String,
  pub value: serde_json::Value,
}
