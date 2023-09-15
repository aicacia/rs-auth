use std::net::{IpAddr, Ipv4Addr};

use actix_web::HttpServer;
use anyhow::Result;

use auth::{
  app::create_app,
  core::{db::create_pool, settings::get_setting},
};

#[actix_web::main]
async fn main() -> Result<()> {
  dotenv::dotenv()?;

  let pool = create_pool().await?;

  let log_level = get_setting(&pool, "log_level").await.to_string();
  env_logger::init_from_env(env_logger::Env::new().default_filter_or(log_level));

  let host = std::env::var("HOST")
    .unwrap_or(
      get_setting(&pool, "server.address")
        .await
        .as_str()
        .unwrap_or_default()
        .to_owned(),
    )
    .parse::<IpAddr>()
    .ok()
    .unwrap_or(IpAddr::from(Ipv4Addr::UNSPECIFIED));

  let port = std::env::var("PORT")
    .unwrap_or(get_setting(&pool, "server.port").await.to_string())
    .parse::<u16>()
    .unwrap_or(80);

  let http_pool = pool.clone();
  HttpServer::new(move || create_app(&http_pool))
    .bind((host, port))?
    .run()
    .await?;

  pool.close().await;

  Ok(())
}
