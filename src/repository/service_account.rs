use crate::core::encryption::verify_password;

#[derive(Clone, sqlx::FromRow)]
pub struct ServiceAccountRow {
  pub id: i64,
  pub client_id: String,
  pub encrypted_client_secret: String,
  pub name: String,
  pub active: i32,
  pub updated_at: i64,
  pub created_at: i64,
}

impl ServiceAccountRow {
  pub fn is_active(&self) -> bool {
    self.active != 0
  }
  pub fn verify(&self, secret: &str) -> Result<bool, argon2::Error> {
    verify_password(secret, &self.encrypted_client_secret)
  }
}

pub async fn get_service_accounts(
  pool: &sqlx::AnyPool,
  limit: Option<usize>,
  offset: Option<usize>,
) -> sqlx::Result<Vec<ServiceAccountRow>> {
  let mut qb = sqlx::QueryBuilder::new("SELECT sa.* FROM service_accounts sa");
  if let Some(limit) = limit {
    qb.push(" LIMIT ").push_bind(limit as i64);
  }
  if let Some(offset) = offset {
    qb.push(" OFFSET ").push_bind(offset as i64);
  }
  qb.build_query_as().fetch_all(pool).await
}

pub async fn get_service_account_by_id(
  pool: &sqlx::AnyPool,
  service_account_id: i64,
) -> sqlx::Result<Option<ServiceAccountRow>> {
  sqlx::query_as(
    r#"SELECT sa.*
    FROM service_accounts sa
    WHERE sa.id = $1
    LIMIT 1;"#,
  )
  .bind(service_account_id)
  .fetch_optional(pool)
  .await
}

pub async fn get_service_account_by_client_id(
  pool: &sqlx::AnyPool,
  client_id: &str,
) -> sqlx::Result<Option<ServiceAccountRow>> {
  sqlx::query_as(
    r#"SELECT sa.*
    FROM service_accounts sa
    WHERE sa.client_id = $1
    LIMIT 1;"#,
  )
  .bind(client_id)
  .fetch_optional(pool)
  .await
}

pub struct CreateServiceAccount {
  pub client_id: String,
  pub encrypted_client_secret: String,
  pub name: String,
}

pub async fn create_service_account(
  pool: &sqlx::AnyPool,
  service_account: CreateServiceAccount,
) -> sqlx::Result<ServiceAccountRow> {
  sqlx::query_as(
    r#"INSERT INTO service_accounts (client_id, encrypted_client_secret, name)
    VALUES ($1, $2, $3)
    RETURNING *;"#,
  )
  .bind(service_account.client_id)
  .bind(service_account.encrypted_client_secret)
  .bind(service_account.name)
  .fetch_one(pool)
  .await
}

#[derive(Default)]
pub struct UpdateServiceAccount {
  pub client_id: Option<String>,
  pub encrypted_client_secret: Option<String>,
  pub name: Option<String>,
  pub active: Option<i64>,
}

pub async fn update_service_account(
  pool: &sqlx::AnyPool,
  service_account_id: i64,
  service_account: UpdateServiceAccount,
) -> sqlx::Result<Option<ServiceAccountRow>> {
  sqlx::query_as(
    r#"UPDATE service_accounts
    SET client_id = COALESCE($1, client_id),
        encrypted_client_secret = COALESCE($2, encrypted_client_secret),
        name = COALESCE($3, name),
        active = COALESCE($4, active),
        updated_at = $5
    WHERE id = $5
    RETURNING *;"#,
  )
  .bind(service_account.client_id)
  .bind(service_account.encrypted_client_secret)
  .bind(service_account.name)
  .bind(service_account.active)
  .bind(service_account_id)
  .bind(chrono::Utc::now().timestamp())
  .fetch_optional(pool)
  .await
}

pub async fn delete_service_account(
  pool: &sqlx::AnyPool,
  service_account_id: i64,
) -> sqlx::Result<Option<ServiceAccountRow>> {
  sqlx::query_as(
    r#"DELETE FROM service_accounts
    WHERE id = $1
    RETURNING *;"#,
  )
  .bind(service_account_id)
  .fetch_optional(pool)
  .await
}
