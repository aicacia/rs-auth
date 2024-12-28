use std::{
  fs::{create_dir_all, File},
  future::Future,
  path::Path,
  pin::Pin,
  sync::atomic::Ordering,
  time::Duration,
};

use sqlx::{migrate::Migrator, Executor};

use super::{atomic_value::AtomicValue, config::get_config};

static POOL: AtomicValue<sqlx::AnyPool> = AtomicValue::empty();

static SQLITE_MIGRATOR: Migrator = sqlx::migrate!("./migrations/sqlite");
static POSTGRESQL_MIGRATOR: Migrator = sqlx::migrate!("./migrations/postgresql");

pub async fn init_pool() -> Result<sqlx::AnyPool, sqlx::Error> {
  let config = get_config();

  log::info!("Creating pool for database: {}", config.database.url);

  if config.database.url.starts_with("sqlite:") {
    let path = Path::new(&config.database.url["sqlite:".len()..]);
    if let Some(parent) = path.parent() {
      if !parent.as_os_str().is_empty() && !parent.exists() {
        log::info!("Creating database directory: {:?}", parent);
        match create_dir_all(parent) {
          Ok(_) => (),
          Err(e) => {
            log::error!("Failed to create database directory: {}", e);
            return Err(sqlx::Error::Io(e));
          }
        }
      }
    }
    if !path.exists() {
      log::info!("Creating database file: {:?}", path);
      match File::create(path) {
        Ok(_) => (),
        Err(e) => {
          log::error!("Failed to create database file: {}", e);
          return Err(sqlx::Error::Io(e));
        }
      }
    }
  }

  let pool = sqlx::any::AnyPoolOptions::new()
    .min_connections(config.database.min_connections)
    .max_connections(config.database.max_connections)
    .acquire_timeout(Duration::from_secs(config.database.acquire_timeout))
    .idle_timeout(Duration::from_secs(config.database.idle_timeout))
    .max_lifetime(Duration::from_secs(config.database.max_lifetime))
    .after_connect(|conn, _meta| {
      Box::pin(async move {
        match conn.backend_name().to_lowercase().as_str() {
          "sqlite" => {
            conn
              .execute(
                "PRAGMA journal_mode = wal; PRAGMA synchronous = normal; PRAGMA foreign_keys = on;",
              )
              .await?;
          }
          _ => (),
        }
        Ok(())
      })
    })
    .connect(&config.database.url)
    .await?;

  POOL.set(pool.clone(), Ordering::SeqCst);

  if config.database.url.starts_with("sqlite:") {
    SQLITE_MIGRATOR.run(&pool).await?;
  } else if config.database.url.starts_with("postgres:") {
    POSTGRESQL_MIGRATOR.run(&pool).await?;
  }

  Ok(pool)
}

pub fn get_pool() -> sqlx::AnyPool {
  assert!(!POOL.is_empty(), "Pool not initialized");
  POOL.get(Ordering::Relaxed)
}

pub async fn run_transaction<T, F>(
  pool: &sqlx::AnyPool,
  transaction_fn: F,
) -> Result<T, sqlx::Error>
where
  F: for<'a> FnOnce(
    &'a mut sqlx::Transaction<'_, sqlx::Any>,
  ) -> Pin<Box<dyn Send + Future<Output = sqlx::Result<T>> + 'a>>,
{
  let mut transaction = pool.begin().await?;
  let result = match transaction_fn(&mut transaction).await {
    Ok(result) => result,
    Err(e) => match transaction.rollback().await {
      Ok(_) => return Err(e),
      Err(e2) => {
        log::error!("Failed to rollback transaction: {}", e2);
        return Err(e);
      }
    },
  };
  transaction.commit().await?;
  Ok(result)
}
