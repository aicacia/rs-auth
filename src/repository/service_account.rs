use crate::core::encryption::verify_password;

#[derive(Clone, sqlx::FromRow)]
pub struct ServiceAccountRow {
  pub id: i64,
  pub client_id: String,
  pub encrypted_secret: String,
  pub name: String,
  pub active: i32,
  pub created_at: i64,
  pub updated_at: i64,
}

impl ServiceAccountRow {
  pub fn is_active(&self) -> bool {
    self.active != 0
  }
  pub fn verify(&self, secret: &str) -> Result<bool, argon2::Error> {
    verify_password(secret, &self.encrypted_secret)
  }
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
