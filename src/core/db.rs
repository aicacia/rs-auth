use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn create_pool() -> Result<Pool<Postgres>> {
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env variable is required");
  let pool = sqlx::Pool::connect(&database_url).await?;
  Ok(pool)
}
