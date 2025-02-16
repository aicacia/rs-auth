use std::{str::FromStr, sync::Arc};

use tokio_util::sync::CancellationToken;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
  core::{
    config::Config,
    database::{close_pool, init_pool},
    error::InternalError,
  },
  service::start_up::init_service_accounts,
};

#[cfg(feature = "completions")]
use super::completions;
use super::{
  args::{CliArgs, CliCommand},
  serve::serve,
};

pub async fn run(args: CliArgs) -> Result<(), InternalError> {
  let config = Arc::new(Config::new(&args.config).await?);

  let level = tracing::Level::from_str(&config.log_level).unwrap_or(tracing::Level::DEBUG);
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!(
          "{}={level},tower_http={level},axum::rejection=trace,sqlx::query={level}",
          env!("CARGO_PKG_NAME"),
          level = level.as_str().to_lowercase()
        )
        .into()
      }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  let pool = init_pool(config.as_ref()).await?;

  init_service_accounts(&pool, config.as_ref(), config.default_application_id).await?;

  let cancellation_token = CancellationToken::new();

  let command_handle = match args.command {
    Some(CliCommand::Serve { .. }) => tokio::spawn(serve(config, pool, cancellation_token.clone())),
    #[cfg(feature = "completions")]
    Some(CliCommand::Completions { shell }) => tokio::spawn(completions::run(shell)),
    None => tokio::spawn(serve(config, pool, cancellation_token.clone())),
  };

  shutdown_signal(cancellation_token).await;

  match command_handle.await {
    Ok(_) => {}
    Err(e) => {
      log::error!("Error: {}", e);
    }
  }

  match close_pool().await {
    Ok(_) => {}
    Err(e) => {
      log::error!("error closing pool: {}", e);
    }
  }

  Ok(())
}

async fn shutdown_signal(cancellation_token: CancellationToken) {
  let ctrl_c = async {
    tokio::signal::ctrl_c()
      .await
      .map_err(|e| InternalError::internal_error().with_application_error(e.to_string()))
  };

  #[cfg(unix)]
  let terminate = async {
    match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
      Ok(mut signal) => match signal.recv().await {
        Some(_) => Ok(()),
        None => Ok(()),
      },
      Err(e) => Err(InternalError::internal_error().with_application_error(e.to_string())),
    }
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  let result = tokio::select! {
    result = ctrl_c => result,
    result = terminate => result,
  };

  match result {
    Ok(_) => log::info!("Shutdown signal received, shutting down"),
    Err(e) => log::error!("error receiving shutdown signal: {}", e),
  }

  cancellation_token.cancel();
}
