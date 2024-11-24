use crate::core::encryption::verify_password;

#[derive(sqlx::FromRow)]
pub struct UserPasswordRow {
  pub id: i64,
  pub user_id: i64,
  pub active: i32,
  pub encrypted_password: String,
  pub created_at: i64,
  pub updated_at: i64,
}

impl UserPasswordRow {
  pub fn is_active(&self) -> bool {
    self.active != 0
  }
  pub fn verify(&self, password: &str) -> Result<bool, argon2::Error> {
    verify_password(password, &self.encrypted_password)
  }
}

pub async fn get_active_user_password_by_user_id(
  pool: &sqlx::AnyPool,
  user_id: i64,
) -> sqlx::Result<Option<UserPasswordRow>> {
  sqlx::query_as(
    r#"SELECT up.*
    FROM user_passwords up
    WHERE up.active AND up.user_id = $1 
    LIMIT 1;"#,
  )
  .bind(user_id)
  .fetch_optional(pool)
  .await
}
