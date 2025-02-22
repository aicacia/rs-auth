#[derive(sqlx::FromRow)]
pub struct TenantRow {
  pub id: i64,
  pub application_id: i64,
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

pub fn from_tenants_query<'a>(
  qb: &mut sqlx::QueryBuilder<'a, sqlx::Any>,
  application_id: i64,
  limit: Option<usize>,
  offset: Option<usize>,
) {
  qb.push(" FROM tenants t");
  qb.push(" WHERE t.application_id = ");
  qb.push(application_id);
  if let Some(limit) = limit {
    qb.push(" LIMIT ").push(limit as i64);
  }
  if let Some(offset) = offset {
    qb.push(" OFFSET ").push(offset as i64);
  }
}

pub async fn get_tenants(
  pool: &sqlx::AnyPool,
  application_id: i64,
  limit: Option<usize>,
  offset: Option<usize>,
) -> sqlx::Result<Vec<TenantRow>> {
  let mut qb = sqlx::QueryBuilder::new("SELECT t.* ");
  from_tenants_query(&mut qb, application_id, limit, offset);
  qb.build_query_as().fetch_all(pool).await
}

pub async fn get_tenant_by_id(
  pool: &sqlx::AnyPool,
  application_id: i64,
  tenant_id: i64,
) -> sqlx::Result<Option<TenantRow>> {
  sqlx::query_as(
    r#"SELECT t.*
    FROM tenants t
    WHERE t.application_id = $1 AND t.id = $2
    LIMIT 1;"#,
  )
  .bind(application_id)
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

pub async fn create_tenant(
  pool: &sqlx::AnyPool,
  application_id: i64,
  tenant: CreateTenant,
) -> sqlx::Result<TenantRow> {
  sqlx::query_as(
    r#"INSERT INTO tenants (
      application_id,
      client_id,
      issuer,
      audience,
      algorithm,
      public_key,
      private_key,
      expires_in_seconds,
      refresh_expires_in_seconds
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    RETURNING *;"#,
  )
  .bind(application_id)
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
  application_id: i64,
  tenant_id: i64,
  tenant: UpdateTenant,
) -> sqlx::Result<Option<TenantRow>> {
  sqlx::query_as(
    r#"UPDATE tenants SET
      client_id = COALESCE($3, client_id),
      issuer = COALESCE($4, issuer),
      audience = COALESCE($5, audience),
      algorithm = COALESCE($6, algorithm),
      public_key = COALESCE($7, public_key),
      private_key = COALESCE($8, private_key),
      expires_in_seconds = COALESCE($9, expires_in_seconds),
      refresh_expires_in_seconds = COALESCE($10, refresh_expires_in_seconds),
      updated_at = $11
    WHERE application_id = $1 AND id = $2
    RETURNING *;"#,
  )
  .bind(application_id)
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
  application_id: i64,
  tenant_id: i64,
) -> sqlx::Result<Option<TenantRow>> {
  sqlx::query_as(
    r#"DELETE FROM tenants
    WHERE application_id = $1 AND id = $2
    RETURNING *;"#,
  )
  .bind(application_id)
  .bind(tenant_id)
  .fetch_optional(pool)
  .await
}
