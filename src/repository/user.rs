use rand::distributions::{Alphanumeric, DistString};

use crate::core::{database::run_transaction, encryption::encrypt_password};

use super::user_info::UserInfoUpdate;

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
  pub user_info: UserInfoUpdate,
}

async fn create_user_internal(
  transaction: &mut sqlx::Transaction<'_, sqlx::Any>,
  params: CreateUser,
) -> sqlx::Result<UserRow> {
  let user: UserRow =
    sqlx::query_as(r#"INSERT INTO users ("username", "active") VALUES ($1, $2) RETURNING *;"#)
      .bind(params.username)
      .bind(true as i32)
      .fetch_one(&mut **transaction)
      .await?;

  sqlx::query(r#"INSERT INTO user_infos 
      ("user_id", "name", "given_name", "family_name", "middle_name", "nickname", "profile_picture", "website", "gender", "birthdate", "zone_info", "locale", "address")
      VALUES 
      ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
      RETURNING *;"#)
    .bind(user.id)
    .bind(params.user_info.name)
    .bind(params.user_info.given_name)
    .bind(params.user_info.family_name)
    .bind(params.user_info.middle_name)
    .bind(params.user_info.nickname)
    .bind(params.user_info.profile_picture)
    .bind(params.user_info.website)
    .bind(params.user_info.gender)
    .bind(params.user_info.birthdate)
    .bind(params.user_info.zone_info)
    .bind(params.user_info.locale)
    .bind(params.user_info.address)
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
          user_info: Default::default(),
        },
      )
      .await?;

      sqlx::query(
        r#"INSERT INTO user_passwords ("user_id", "encrypted_password") VALUES ($1, $2);"#,
      )
      .bind(user.id)
      .bind(encrypted_password)
      .execute(&mut **transaction)
      .await?;

      Ok(user)
    })
  })
  .await
}

pub struct CreateUserWithOAuth2 {
  pub active: bool,
  pub provider: String,
  pub email: String,
  pub email_verified: bool,
  pub user_info: UserInfoUpdate,
}

pub async fn create_user_with_oauth2(
  pool: &sqlx::AnyPool,
  params: CreateUserWithOAuth2,
) -> sqlx::Result<UserRow> {
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let mut username: String = params.email.split('@').next().unwrap_or_default().trim().to_string();
      if username.is_empty() {
        return Err(sqlx::Error::Encode("Failed to convert email into username".into()));
      }

      while username_used(transaction, &username).await? {
        username.push_str(&Alphanumeric.sample_string(&mut rand::thread_rng(), 2));
      }

      let user = create_user_internal(
        transaction,
        CreateUser {
          username,
          active: true,
          user_info: params.user_info
        },
      )
      .await?;

      sqlx::query(
        r#"INSERT INTO user_oauth2_providers ("user_id", "provider", "email") VALUES ($1, $2, $3);"#,
      )
      .bind(user.id)
      .bind(&params.provider)
      .bind(&params.email)
      .execute(&mut **transaction)
      .await?;

      sqlx::query(
        r#"INSERT INTO user_emails ("user_id", "email", "verified", "primary") VALUES ($1, $2, $3, TRUE);"#,
      )
      .bind(user.id)
      .bind(&params.email)
      .bind(params.email_verified)
      .execute(&mut **transaction)
      .await?;

      Ok(user)
    })
  })
  .await
}

async fn username_used(
  transaction: &mut sqlx::Transaction<'_, sqlx::Any>,
  username: &str,
) -> sqlx::Result<bool> {
  let user: Option<UserRow> = sqlx::query_as(
    r#"SELECT u.*
    FROM users u
    WHERE u.username = $1
    LIMIT 1;"#,
  )
  .bind(username)
  .fetch_optional(&mut **transaction)
  .await?;
  Ok(user.is_some())
}
