use std::{net::SocketAddr, sync::Arc};

use crate::{
  core::{config::Config, error::InternalError},
  router::{create_router, RouterState},
};
use tokio_util::sync::CancellationToken;

pub async fn serve(
  config: Arc<Config>,
  pool: sqlx::AnyPool,
  cancellation_token: CancellationToken,
) -> Result<(), InternalError> {
  let router = create_router(RouterState {
    config: config.clone(),
    pool: pool.clone(),
  });
  let serve_shutdown_signal = async move {
    cancellation_token.cancelled().await;
  };

  let listener = tokio::net::TcpListener::bind(&SocketAddr::from((
    config.server.address,
    config.server.port,
  )))
  .await?;
  let local_addr = listener.local_addr()?;
  log::info!("Listening on {}", local_addr);
  axum::serve(
    listener,
    router.into_make_service_with_connect_info::<SocketAddr>(),
  )
  .with_graceful_shutdown(serve_shutdown_signal)
  .await?;
  Ok(())
}
