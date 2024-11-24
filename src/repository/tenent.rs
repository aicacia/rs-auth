use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct TenentRow {
  pub id: i64,
  pub client_id: String,
  pub issuer: String,
  pub audience: String,
  pub algorithm: String,
  pub public_key: Option<String>,
  pub private_key: String,
  pub expires_in_seconds: i64,
  pub refresh_expires_in_seconds: i64,
  pub created_at: i64,
  pub updated_at: i64,
}

pub async fn get_tenent_by_id(
  pool: &sqlx::AnyPool,
  tenent_id: i64,
) -> sqlx::Result<Option<TenentRow>> {
  sqlx::query_as(
    r#"SELECT t.*
    FROM tenents t
    WHERE t.id = $1
    LIMIT 1;"#,
  )
  .bind(tenent_id)
  .fetch_optional(pool)
  .await
}

pub async fn get_tenent_by_client_id(
  pool: &sqlx::AnyPool,
  tenent_client_id: &Uuid,
) -> sqlx::Result<Option<TenentRow>> {
  sqlx::query_as(
    r#"SELECT t.*
    FROM tenents t
    WHERE t.client_id = $1
    LIMIT 1;"#,
  )
  .bind(tenent_client_id.to_string())
  .fetch_optional(pool)
  .await
}
