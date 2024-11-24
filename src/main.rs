use std::{net::SocketAddr, str::FromStr};

use auth::{
  core::{config::init_config, database::init_pool, error::Errors},
  router::{create_router, RouterState},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Errors> {
  dotenv::dotenv().ok();
  sqlx::any::install_default_drivers();

  let config = init_config().await?;

  let level = tracing::Level::from_str(&config.log_level).unwrap_or(tracing::Level::DEBUG);
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!(
          "{}={level},tower_http={level},axum::rejection=trace",
          env!("CARGO_PKG_NAME"),
          level = level.as_str().to_lowercase()
        )
        .into()
      }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  let pool = init_pool().await?;

  let router = create_router(RouterState { pool: pool.clone() });

  let listener = tokio::net::TcpListener::bind(&SocketAddr::from((
    config.server.address,
    config.server.port,
  )))
  .await?;
  log::info!("Listening on {}", listener.local_addr()?);
  axum::serve(listener, router).await?;

  pool.close().await;

  Ok(())
}
