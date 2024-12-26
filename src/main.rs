use std::{fs, net::SocketAddr, str::FromStr};

use auth::{
  core::{
    config::init_config,
    database::init_pool,
    encryption,
    error::{Errors, DATEBASE_ERROR, INTERNAL_ERROR},
  },
  model::service_account::ServiceAccount,
  repository::{self, service_account::get_service_accounts},
  router::{create_router, RouterState},
};
use sqlx::Executor;
use tokio_util::sync::CancellationToken;
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

  init_service_account(&pool).await?;

  let router = create_router(RouterState { pool: pool.clone() });

  let cancellation_token = CancellationToken::new();

  let serve_cancellation_token = cancellation_token.clone();
  let serve_shutdown_signal = async move {
    serve_cancellation_token.cancelled().await;
  };
  let serve_handle = tokio::spawn(async move {
    let listener = match tokio::net::TcpListener::bind(&SocketAddr::from((
      config.server.address,
      config.server.port,
    )))
    .await
    {
      Ok(listener) => listener,
      Err(e) => {
        log::error!("Error binding to address: {}", e);
        return;
      }
    };
    let local_addr = match listener.local_addr() {
      Ok(addr) => addr,
      Err(e) => {
        log::error!("Error getting local address: {}", e);
        return;
      }
    };
    log::info!("Listening on {}", local_addr);
    match axum::serve(
      listener,
      router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(serve_shutdown_signal)
    .await
    {
      Ok(_) => {}
      Err(e) => {
        log::error!("Error serving: {}", e);
      }
    }
  });

  let pool_cleanup_cancellation_token = cancellation_token.clone();
  let pool_cleanup_handle = tokio::spawn(async move {
    pool_cleanup_cancellation_token.cancelled().await;

    match pool.acquire().await {
      Ok(conn) => match conn.backend_name() {
        "sqlite" => {
          log::info!("Optimizing database");
          match pool
            .execute("PRAGMA analysis_limit=400; PRAGMA optimize;")
            .await
          {
            Ok(_) => {
              log::info!("Optimized database");
            }
            Err(e) => {
              log::error!("Error optimizing database: {}", e);
            }
          }
        }
        _ => {}
      },
      Err(e) => {
        log::error!("Error acquiring connection: {}", e);
      }
    }
    pool.close().await;
  });

  shutdown_signal(cancellation_token).await;

  match serve_handle.await {
    Ok(_) => {}
    Err(e) => {
      log::error!("Error serving: {}", e);
    }
  }
  match pool_cleanup_handle.await {
    Ok(_) => {}
    Err(e) => {
      log::error!("Error cleaning up pool: {}", e);
    }
  }

  Ok(())
}

async fn shutdown_signal(cancellation_token: CancellationToken) {
  let ctrl_c = async {
    tokio::signal::ctrl_c()
      .await
      .map_err(|e| Errors::internal_error().with_application_error(e.to_string()))
  };

  #[cfg(unix)]
  let terminate = async {
    match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
      Ok(mut signal) => match signal.recv().await {
        Some(_) => Ok(()),
        None => Ok(()),
      },
      Err(e) => Err(Errors::internal_error().with_application_error(e.to_string())),
    }
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  let result = tokio::select! {
    result = ctrl_c => result,
    result = terminate => result,
  };

  match result {
    Ok(_) => log::info!("Signal received, shutting down"),
    Err(e) => log::error!("Error receiving signal: {}", e),
  }

  cancellation_token.cancel();
}

async fn init_service_account(pool: &sqlx::AnyPool) -> Result<(), Errors> {
  let service_accounts = match get_service_accounts(pool, None, None).await {
    Ok(service_accounts) => service_accounts,
    Err(e) => {
      log::error!("Error getting service accounts: {}", e);
      return Err(Errors::internal_error().with_application_error(DATEBASE_ERROR));
    }
  };
  if !service_accounts.is_empty() {
    return Ok(());
  }
  log::info!("No service accounts found, creating admin service account");
  let client_id = uuid::Uuid::new_v4();
  let client_secret = uuid::Uuid::new_v4();
  let encrypted_client_secret = match encryption::encrypt_password(&client_secret.to_string()) {
    Ok(encrypted_client_secret) => encrypted_client_secret,
    Err(e) => {
      log::error!("Error encrypting client secret: {}", e);
      return Err(Errors::internal_error().with_application_error(DATEBASE_ERROR));
    }
  };
  let service_account_row = match repository::service_account::create_service_account(
    pool,
    repository::service_account::CreateServiceAccount {
      client_id: client_id.to_string(),
      encrypted_client_secret,
      name: "Admin".to_owned(),
    },
  )
  .await
  {
    Ok(row) => row,
    Err(e) => {
      log::error!("Error creating service account: {}", e);
      return Err(Errors::internal_error().with_application_error(DATEBASE_ERROR));
    }
  };
  let mut service_account = ServiceAccount::from(service_account_row);
  service_account.client_secret = Some(client_secret.clone());
  let service_account_json_string = match serde_json::to_string_pretty(&service_account) {
    Ok(json) => json,
    Err(e) => {
      log::error!("Error serializing service account: {}", e);
      return Err(Errors::internal_error().with_application_error(INTERNAL_ERROR));
    }
  };
  match fs::write(
    "./auth-admin-service-account.json",
    service_account_json_string,
  ) {
    Ok(_) => {}
    Err(e) => {
      log::error!("Error writing service account to file: {}", e);
      return Err(Errors::internal_error().with_application_error(INTERNAL_ERROR));
    }
  }
  log::info!("Service account created, see auth-admin-service-account.json");
  Ok(())
}
