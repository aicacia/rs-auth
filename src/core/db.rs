use anyhow::Result;
use futures::Future;
use serde::de::DeserializeOwned;
use sqlx::{postgres::PgListener, Pool, Postgres};

pub async fn create_pool() -> Result<Pool<Postgres>> {
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env variable is required");
  let pool = sqlx::Pool::connect(&database_url).await?;
  Ok(pool)
}

pub async fn start_listening<T, F, FR>(
  pool: Pool<Postgres>,
  channels: Vec<&str>,
  call_back: F,
) -> Result<()>
where
  T: DeserializeOwned + 'static,
  F: Fn(T, Pool<Postgres>) -> FR,
  FR: Future<Output = Result<()>>,
{
  log::info!("Listening for notifications on channels: {:?}", channels);
  let mut listener = PgListener::connect_with(&pool).await?;
  listener.listen_all(channels).await?;
  tokio::pin!(listener);
  loop {
    tokio::select! {
      result = listener.try_recv() => match result {
        Ok(Some(notification)) => {
          log::debug!("Received notification: {:?}", notification);
          match serde_json::from_str::<T>(notification.payload()) {
            Ok(payload) => match call_back(payload, pool.clone()).await {
              Ok(_) => {},
              Err(e) => {
                log::error!("Error calling callback {}", e);
              }
            },
            Err(e) => {
              log::error!("Error parsing payload {}", e);
            }
          }
        },
        Ok(None) => {},
        Err(e) => {
          log::error!("Error receiving notification {}", e);
          break;
        }
      },
      result = tokio::signal::ctrl_c() => match result {
        Ok(_) => {
          log::info!("Received ctrl+c signal");
          break;
        },
        Err(e) => {
          log::error!("Error waiting on ctrl+c signal {}", e);
          break;
        }
      }
    }
  }
  Ok(())
}
