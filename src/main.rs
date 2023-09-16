use std::net::{IpAddr, Ipv4Addr};

use actix_web::HttpServer;
use anyhow::Result;

use auth::{
  app::create_app,
  core::{
    config::{get_config, init_config},
    db::create_pool,
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

  let server_result = server.await;
  pool.close().await;

  match server_result {
    Ok(_) => Ok(()),
    Err(e) => Err(anyhow::Error::from(e)),
  }
}
