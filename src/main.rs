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

  let listener = tokio::net::TcpListener::bind(&SocketAddr::from((
    config.server.address,
    config.server.port,
  )))
  .await?;
  log::info!("Listening on {}", listener.local_addr()?);
  axum::serve(listener, router).await?;

  // TODO: make this run on shutdown
  match pool.acquire().await {
    Ok(conn) => match conn.backend_name() {
      "sqlite" => {
        log::info!("Optimizing database");
        pool
          .execute("PRAGMA analysis_limit=400; PRAGMA optimize;")
          .await?;
      }
      _ => {}
    },
    Err(e) => {
      log::error!("Error acquiring connection: {}", e);
    }
  }
  pool.close().await;

  Ok(())
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
