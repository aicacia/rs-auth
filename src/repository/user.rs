use crate::core::{database::run_transaction, encryption::encrypt_password};

#[derive(Clone, sqlx::FromRow)]
pub struct UserRow {
  pub id: i64,
  pub username: String,
  pub active: i32,
  pub created_at: i64,
  pub updated_at: i64,
}

impl UserRow {
  pub fn is_active(&self) -> bool {
    self.active != 0
  }
}

pub async fn get_user_by_id(pool: &sqlx::AnyPool, user_id: i64) -> sqlx::Result<Option<UserRow>> {
  sqlx::query_as(
    r#"SELECT u.*
    FROM users u
    WHERE u.id = $1
    LIMIT 1;"#,
  )
  .bind(user_id)
  .fetch_optional(pool)
  .await
}

pub async fn get_user_by_username(
  pool: &sqlx::AnyPool,
  username: &str,
) -> sqlx::Result<Option<UserRow>> {
  sqlx::query_as(
    r#"SELECT u.*
    FROM users u
    WHERE u.username = $1
    LIMIT 1;"#,
  )
  .bind(username)
  .fetch_optional(pool)
  .await
}
pub async fn create_user(pool: &sqlx::AnyPool, params: CreateUser) -> sqlx::Result<UserRow> {
  run_transaction(pool, |transaction| {
    Box::pin(async move { create_user_internal(transaction, params).await })
  })
  .await
}

pub struct CreateUser {
  pub username: String,
  pub active: bool,
}

async fn create_user_internal(
  transaction: &mut sqlx::Transaction<'_, sqlx::Any>,
  params: CreateUser,
) -> sqlx::Result<UserRow> {
  let user: UserRow =
    sqlx::query_as(r#"INSERT INTO users (username, active) VALUES ($1, $2) RETURNING *;"#)
      .bind(params.username)
      .bind(true as i32)
      .fetch_one(&mut **transaction)
      .await?;

  sqlx::query(r#"INSERT INTO user_infos (user_id) VALUES ($1);"#)
    .bind(user.id)
    .execute(&mut **transaction)
    .await?;

  Ok(user)
}

pub struct CreateUserWithPassword {
  pub username: String,
  pub password: String,
}

pub async fn create_user_with_password(
  pool: &sqlx::AnyPool,
  params: CreateUserWithPassword,
) -> sqlx::Result<UserRow> {
  let encrypted_password = match encrypt_password(&params.password) {
    Ok(encrypted_password) => encrypted_password,
    Err(e) => {
      return Err(sqlx::Error::Encode(
        format!("Failed to encrypt password: {}", e).into(),
      ))
    }
  };
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let user = create_user_internal(
        transaction,
        CreateUser {
          username: params.username,
          active: true,
        },
      )
      .await?;

      sqlx::query(r#"INSERT INTO user_passwords (user_id, encrypted_password) VALUES ($1, $2);"#)
        .bind(user.id)
        .bind(encrypted_password)
        .execute(&mut **transaction)
        .await?;

      Ok(user)
    })
  })
  .await
}

pub struct CreateUserWithOAuth2ProviderAndEmail {
  pub active: bool,
  pub provider: String,
  pub email: String,
  pub email_verified: bool,
}

pub async fn create_user_with_oauth2_provider_and_email(
  pool: &sqlx::AnyPool,
  params: CreateUserWithOAuth2ProviderAndEmail,
) -> sqlx::Result<UserRow> {
  Err(sqlx::Error::RowNotFound)
}
