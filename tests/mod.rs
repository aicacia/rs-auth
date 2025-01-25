use std::{path::Path, str::FromStr, sync::Arc};

use auth::{
  core::{config::Config, database::init_pool, error::Errors},
  router::{create_router, RouterState},
};
use axum::{body::Body, Router};
use http::{Request, StatusCode};
use scopeguard::defer;
use tokio::{fs::remove_file, runtime::Handle, task::block_in_place};
use tower::ServiceExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::test(flavor = "multi_thread")]
async fn not_found() -> Result<(), Errors> {
  let (router, config, pool) = setup().await?;
  defer! { teardown(config, pool) }

  let response = router
    .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
    .await
    .unwrap();

  assert_eq!(response.status(), StatusCode::NOT_FOUND);

  Ok(())
}

pub async fn setup() -> Result<(Router, Arc<Config>, sqlx::AnyPool), Errors> {
  dotenvy::from_path("./.env.test").ok();
  sqlx::any::install_default_drivers();

  let mut config = Config::new("./config.test.json").await?;
  config.database.url = format!("sqlite:tests/.dbs/auth-{}-test.db", uuid::Uuid::new_v4());
  let config = Arc::new(config);

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

  let pool = init_pool(config.as_ref()).await?;
  let router = create_router(RouterState {
    config: config.clone(),
    pool: pool.clone(),
  });
  Ok((router, config, pool))
}

pub fn teardown(config: Arc<Config>, pool: sqlx::AnyPool) {
  block_in_place(move || {
    Handle::current().block_on(async move {
      pool.close().await;
      if config.database.url.starts_with("sqlite:") {
        let path = Path::new(&config.database.url["sqlite:".len()..]);
        remove_file(path)
          .await
          .expect(&format!("failed to delete: {:?}", path));
      }
    });
  });
}
