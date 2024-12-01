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

pub async fn get_tenents(
  pool: &sqlx::AnyPool,
  limit: usize,
  offset: usize,
) -> sqlx::Result<Vec<TenentRow>> {
  sqlx::query_as(r#"SELECT t.* FROM tenents t LIMIT $1 OFFSET $2;"#)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await
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

pub struct CreateTenent {
  pub client_id: String,
  pub issuer: String,
  pub audience: String,
  pub algorithm: String,
  pub public_key: Option<String>,
  pub private_key: String,
  pub expires_in_seconds: i64,
  pub refresh_expires_in_seconds: i64,
}

pub async fn create_tenent(pool: &sqlx::AnyPool, tenent: CreateTenent) -> sqlx::Result<TenentRow> {
  sqlx::query_as(
    r#"INSERT INTO tenents (
      client_id,
      issuer,
      audience,
      algorithm,
      public_key,
      private_key,
      expires_in_seconds,
      refresh_expires_in_seconds
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    RETURNING *;"#,
  )
  .bind(tenent.client_id)
  .bind(tenent.issuer)
  .bind(tenent.audience)
  .bind(tenent.algorithm)
  .bind(tenent.public_key)
  .bind(tenent.private_key)
  .bind(tenent.expires_in_seconds)
  .bind(tenent.refresh_expires_in_seconds)
  .fetch_one(pool)
  .await
}

pub struct UpdateTenent {
  pub client_id: Option<String>,
  pub issuer: Option<String>,
  pub audience: Option<String>,
  pub algorithm: Option<String>,
  pub public_key: Option<String>,
  pub private_key: Option<String>,
  pub expires_in_seconds: Option<i64>,
  pub refresh_expires_in_seconds: Option<i64>,
}

pub async fn update_tenent(
  pool: &sqlx::AnyPool,
  tenent_id: i64,
  tenent: UpdateTenent,
) -> sqlx::Result<Option<TenentRow>> {
  sqlx::query_as(
    r#"UPDATE tenents SET
      client_id = COALESCE($2, client_id),
      issuer = COALESCE($3, issuer),
      audience = COALESCE($4, audience),
      algorithm = COALESCE($5, algorithm),
      public_key = COALESCE($6, public_key),
      private_key = COALESCE($7, private_key),
      expires_in_seconds = COALESCE($8, expires_in_seconds),
      refresh_expires_in_seconds = COALESCE($9, refresh_expires_in_seconds),
      updated_at = $10
    WHERE id = $1
    RETURNING *;"#,
  )
  .bind(tenent_id)
  .bind(tenent.client_id)
  .bind(tenent.issuer)
  .bind(tenent.audience)
  .bind(tenent.algorithm)
  .bind(tenent.public_key)
  .bind(tenent.private_key)
  .bind(tenent.expires_in_seconds)
  .bind(tenent.refresh_expires_in_seconds)
  .bind(chrono::Utc::now().timestamp())
  .fetch_optional(pool)
  .await
}

pub async fn delete_tenent(
  pool: &sqlx::AnyPool,
  tenent_id: i64,
) -> sqlx::Result<Option<TenentRow>> {
  sqlx::query_as(
    r#"DELETE FROM tenents
    WHERE id = $1
    RETURNING *;"#,
  )
  .bind(tenent_id)
  .fetch_optional(pool)
  .await
}
