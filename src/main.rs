use std::net::{IpAddr, Ipv4Addr};

use actix_web::HttpServer;
use anyhow::Result;
use futures::join;

use auth::{
  app::create_app,
  core::{
    config::{config_listener, get_config, init_config},
    db::create_pool,
  },
};

#[actix_web::main]
async fn main() -> Result<()> {
  dotenv::dotenv()?;

  let pool = create_pool().await?;
  init_config(&pool).await?;

  let config = get_config();

  env_logger::init_from_env(env_logger::Env::new().default_filter_or(&config.log_level));

  let host = std::env::var("HOST")
    .unwrap_or_default()
    .parse::<IpAddr>()
    .ok()
    .unwrap_or(
      config
        .server
        .address
        .unwrap_or(IpAddr::from(Ipv4Addr::UNSPECIFIED)),
    );

  let port = std::env::var("PORT")
    .unwrap_or_default()
    .parse::<u16>()
    .ok()
    .unwrap_or(config.server.port);

  let http_pool = pool.clone();
  let server = HttpServer::new(move || create_app(&http_pool))
    .bind((host, port))?
    .run();

  let server_handle = tokio::spawn(server);
  let listener_handle = tokio::spawn(config_listener(pool.clone()));

  let result: Result<()> = match join!(listener_handle, server_handle) {
    (Ok(_), Ok(_)) => Ok(()),
    (Err(e), Ok(_)) => Err(e.into()),
    (Ok(_), Err(e)) => Err(e.into()),
    (Err(e), Err(e2)) => Err(anyhow::anyhow!("{:?} {:?}", e, e2)),
  };

  pool.close().await;

  result
}
