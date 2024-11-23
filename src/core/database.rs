use std::{future::Future, pin::Pin, time::Duration};

use super::config::get_config;

pub async fn create_pool() -> Result<sqlx::AnyPool, sqlx::Error> {
  let config = get_config();

  let pool = sqlx::any::AnyPoolOptions::new()
    .min_connections(config.database.min_connections)
    .max_connections(config.database.max_connections)
    .acquire_timeout(Duration::from_secs(config.database.acquire_timeout))
    .idle_timeout(Duration::from_secs(config.database.idle_timeout))
    .max_lifetime(Duration::from_secs(config.database.max_lifetime))
    .connect(&config.database.url)
    .await?;

  Ok(pool)
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
