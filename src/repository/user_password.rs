use crate::core::{
  config::get_config,
  database::run_transaction,
  encryption::{encrypt_password, verify_password},
};

#[derive(sqlx::FromRow)]
pub struct UserPasswordRow {
  pub id: i64,
  pub user_id: i64,
  pub active: i32,
  pub encrypted_password: String,
  pub updated_at: i64,
  pub created_at: i64,
}

impl UserPasswordRow {
  pub fn is_active(&self) -> bool {
    self.active != 0
  }
  pub fn verify(&self, password: &str) -> Result<bool, argon2::Error> {
    verify_password(password, &self.encrypted_password)
  }
}

pub async fn get_user_active_password_by_user_id(
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

pub async fn create_user_password(
  pool: &sqlx::AnyPool,
  user_id: i64,
  password: &str,
) -> sqlx::Result<UserPasswordRow> {
  let encrypted_password = match encrypt_password(password) {
    Ok(encrypted_password) => encrypted_password,
    Err(e) => {
      return Err(sqlx::Error::Encode(
        format!("Failed to encrypt password: {}", e).into(),
      ));
    }
  };
  let current_password = password.to_owned();
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let previous_passwords: Vec<UserPasswordRow> = sqlx::query_as(
        r#"SELECT up.* 
            FROM user_passwords up
            WHERE "user_id" = $1 
            ORDER BY "created_at" DESC
            LIMIT $2;"#,
      )
      .bind(user_id)
      .bind(get_config().password.history as i64)
      .fetch_all(&mut **transaction)
      .await?;

      for previous_password in previous_passwords {
        if previous_password.verify(&current_password).unwrap_or(false) {
          return Err(sqlx::Error::Configuration("password_already_used".into()));
        }
      }

      sqlx::query(
        r#"UPDATE user_passwords SET "active" = 0 WHERE "user_id" = $1 AND "active" = 1;"#,
      )
      .bind(user_id)
      .execute(&mut **transaction)
      .await?;

      sqlx::query_as(
        r#"INSERT INTO user_passwords ("user_id", "encrypted_password") VALUES ($1, $2) RETURNING *;"#,
      )
      .bind(user_id)
      .bind(encrypted_password)
      .fetch_one(&mut **transaction)
      .await
    })
  })
  .await
}
