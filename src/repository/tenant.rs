#[derive(sqlx::FromRow)]
pub struct TenantRow {
  pub id: i64,
  pub client_id: String,
  pub issuer: String,
  pub audience: Option<String>,
  pub algorithm: String,
  pub public_key: Option<String>,
  pub private_key: String,
  pub expires_in_seconds: i64,
  pub refresh_expires_in_seconds: i64,
  pub updated_at: i64,
  pub created_at: i64,
}

pub async fn get_tenants(
  pool: &sqlx::AnyPool,
  limit: usize,
  offset: usize,
) -> sqlx::Result<Vec<TenantRow>> {
  sqlx::query_as(r#"SELECT t.* FROM tenants t LIMIT $1 OFFSET $2;"#)
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await
}

pub async fn get_tenant_by_id(
  pool: &sqlx::AnyPool,
  tenant_id: i64,
) -> sqlx::Result<Option<TenantRow>> {
  sqlx::query_as(
    r#"SELECT t.*
    FROM tenants t
    WHERE t.id = $1
    LIMIT 1;"#,
  )
  .bind(tenant_id)
  .fetch_optional(pool)
  .await
}

pub async fn get_tenant_by_client_id(
  pool: &sqlx::AnyPool,
  tenant_client_id: &str,
) -> sqlx::Result<Option<TenantRow>> {
  sqlx::query_as(
    r#"SELECT t.*
    FROM tenants t
    WHERE t.client_id = $1
    LIMIT 1;"#,
  )
  .bind(tenant_client_id)
  .fetch_optional(pool)
  .await
}

pub struct CreateTenant {
  pub client_id: String,
  pub issuer: String,
  pub audience: String,
  pub algorithm: String,
  pub public_key: Option<String>,
  pub private_key: String,
  pub expires_in_seconds: i64,
  pub refresh_expires_in_seconds: i64,
}

pub async fn create_tenant(pool: &sqlx::AnyPool, tenant: CreateTenant) -> sqlx::Result<TenantRow> {
  sqlx::query_as(
    r#"INSERT INTO tenants (
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
  .bind(tenant.client_id)
  .bind(tenant.issuer)
  .bind(tenant.audience)
  .bind(tenant.algorithm)
  .bind(tenant.public_key)
  .bind(tenant.private_key)
  .bind(tenant.expires_in_seconds)
  .bind(tenant.refresh_expires_in_seconds)
  .fetch_one(pool)
  .await
}

pub struct UpdateTenant {
  pub client_id: Option<String>,
  pub issuer: Option<String>,
  pub audience: Option<String>,
  pub algorithm: Option<String>,
  pub public_key: Option<String>,
  pub private_key: Option<String>,
  pub expires_in_seconds: Option<i64>,
  pub refresh_expires_in_seconds: Option<i64>,
}

pub async fn update_tenant(
  pool: &sqlx::AnyPool,
  tenant_id: i64,
  tenant: UpdateTenant,
) -> sqlx::Result<Option<TenantRow>> {
  sqlx::query_as(
    r#"UPDATE tenants SET
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
  .bind(tenant_id)
  .bind(tenant.client_id)
  .bind(tenant.issuer)
  .bind(tenant.audience)
  .bind(tenant.algorithm)
  .bind(tenant.public_key)
  .bind(tenant.private_key)
  .bind(tenant.expires_in_seconds)
  .bind(tenant.refresh_expires_in_seconds)
  .bind(chrono::Utc::now().timestamp())
  .fetch_optional(pool)
  .await
}

pub async fn delete_tenant(
  pool: &sqlx::AnyPool,
  tenant_id: i64,
) -> sqlx::Result<Option<TenantRow>> {
  sqlx::query_as(
    r#"DELETE FROM tenants
    WHERE id = $1
    RETURNING *;"#,
  )
  .bind(tenant_id)
  .fetch_optional(pool)
  .await
}
