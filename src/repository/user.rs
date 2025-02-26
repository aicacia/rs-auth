use rand::distr::{Alphanumeric, SampleString};

use crate::core::{config::Config, database::run_transaction, encryption::encrypt_password};

use super::user_info::UserInfoUpdate;

#[derive(Clone, sqlx::FromRow)]
pub struct UserRow {
  pub id: i64,
  pub application_id: i64,
  pub username: String,
  pub active: i32,
  pub updated_at: i64,
  pub created_at: i64,
}

impl UserRow {
  pub fn is_active(&self) -> bool {
    self.active != 0
  }
}

pub fn from_users_query<'a>(
  qb: &mut sqlx::QueryBuilder<'a, sqlx::Any>,
  application_id: i64,
  limit: Option<usize>,
  offset: Option<usize>,
) {
  qb.push(" FROM users u");
  qb.push(" WHERE u.application_id = ");
  qb.push(application_id);
  qb.push(" ORDER BY u.updated_at DESC ");
  if let Some(limit) = limit {
    qb.push(" LIMIT ").push(limit as i64);
  }
  if let Some(offset) = offset {
    qb.push(" OFFSET ").push(offset as i64);
  }
}

pub async fn get_users(
  pool: &sqlx::AnyPool,
  application_id: i64,
  limit: Option<usize>,
  offset: Option<usize>,
) -> sqlx::Result<Vec<UserRow>> {
  let mut qb = sqlx::QueryBuilder::new("SELECT u.*");
  from_users_query(&mut qb, application_id, limit, offset);
  qb.build_query_as().fetch_all(pool).await
}

pub async fn get_user_by_id(
  pool: &sqlx::AnyPool,
  application_id: i64,
  user_id: i64,
) -> sqlx::Result<Option<UserRow>> {
  sqlx::query_as(
    r#"SELECT u.*
    FROM users u
    WHERE u.application_id = $1 AND u.id = $2
    LIMIT 1;"#,
  )
  .bind(application_id)
  .bind(user_id)
  .fetch_optional(pool)
  .await
}

pub async fn get_user_by_username(
  pool: &sqlx::AnyPool,
  application_id: i64,
  username: &str,
) -> sqlx::Result<Option<UserRow>> {
  sqlx::query_as(
    r#"SELECT u.*
    FROM users u
    WHERE u.application_id = $1 AND u.username = $2
    LIMIT 1;"#,
  )
  .bind(application_id)
  .bind(username)
  .fetch_optional(pool)
  .await
}

pub async fn get_user_by_username_or_primary_email(
  pool: &sqlx::AnyPool,
  application_id: i64,
  username_or_email: &str,
) -> sqlx::Result<Option<UserRow>> {
  sqlx::query_as(
    r#"SELECT u.*
    FROM users u
    LEFT JOIN user_emails ue ON ue.user_id = u.id
    WHERE u.application_id = $1 AND u.username = $2 OR (ue.email = $2 AND ue."primary" = 1)
    LIMIT 1;"#,
  )
  .bind(application_id)
  .bind(username_or_email)
  .fetch_optional(pool)
  .await
}

pub async fn create_user(
  pool: &sqlx::AnyPool,
  application_id: i64,
  params: CreateUser,
) -> sqlx::Result<UserRow> {
  run_transaction(pool, |transaction| {
    Box::pin(async move { create_user_internal(transaction, application_id, params).await })
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
  application_id: i64,
  params: CreateUser,
) -> sqlx::Result<UserRow> {
  let user: UserRow =
    sqlx::query_as(r#"INSERT INTO users ("application_id", "username", "active") VALUES ($1, $2, $3) RETURNING *;"#)
      .bind(application_id)
      .bind(params.username)
      .bind(true)
      .fetch_one(&mut **transaction)
      .await?;

  sqlx::query(
    r#"INSERT INTO user_configs 
      ("user_id")
      VALUES 
      ($1)
      RETURNING *;"#,
  )
  .bind(user.id)
  .execute(&mut **transaction)
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
  config: &Config,
  application_id: i64,
  params: CreateUserWithPassword,
) -> sqlx::Result<UserRow> {
  let encrypted_password = match encrypt_password(config, &params.password) {
    Ok(encrypted_password) => encrypted_password,
    Err(e) => {
      return Err(sqlx::Error::Encode(
        format!("Failed to encrypt password: {}", e).into(),
      ));
    }
  };
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let user = create_user_internal(
        transaction,
        application_id,
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
  pub tenant_oauth2_provider_id: i64,
  pub email: String,
  pub email_verified: bool,
  pub phone_number: Option<String>,
  pub phone_number_verified: bool,
  pub user_info: UserInfoUpdate,
}

pub async fn create_user_with_oauth2(
  pool: &sqlx::AnyPool,
  application_id: i64,
  params: CreateUserWithOAuth2,
) -> sqlx::Result<UserRow> {
  run_transaction(pool, |transaction| {
    Box::pin(async move {
      let mut username: String;
      if let Some((email_name, _)) = params.email.split_once('@') {
        if email_name.is_empty() {
          return Err(sqlx::Error::Encode("Failed to convert email into username".into()));
        }
        username = email_name.to_owned();
      } else {
        return Err(sqlx::Error::Encode("Failed to convert email into username".into()));
      }

      while username_used(transaction, application_id, &username).await? {
        username.push_str(&Alphanumeric.sample_string(&mut rand::rng(), 2));
      }

      let user = create_user_internal(
        transaction,
        application_id,
        CreateUser {
          username,
          active: true,
          user_info: params.user_info
        },
      )
      .await?;

      sqlx::query(
        r#"INSERT INTO user_oauth2_providers ("user_id", "tenant_oauth2_provider_id", "email") VALUES ($1, $2, $3);"#,
      )
      .bind(user.id)
      .bind(&params.tenant_oauth2_provider_id)
      .bind(&params.email)
      .execute(&mut **transaction)
      .await?;

      sqlx::query(
        r#"INSERT INTO user_emails ("user_id", "email", "verified", "primary") VALUES ($1, $2, $3, 1);"#,
      )
      .bind(user.id)
      .bind(&params.email)
      .bind(params.email_verified)
      .execute(&mut **transaction)
      .await?;

      if let Some(phone_number) = params.phone_number.as_ref() {
        sqlx::query(
          r#"INSERT INTO user_phone_numbers ("user_id", "phone_number", "verified", "primary") VALUES ($1, $2, $3, 1);"#,
        )
        .bind(user.id)
        .bind(phone_number)
        .bind(params.phone_number_verified)
        .execute(&mut **transaction)
        .await?;
      }

      Ok(user)
    })
  })
  .await
}

async fn username_used(
  transaction: &mut sqlx::Transaction<'_, sqlx::Any>,
  application_id: i64,
  username: &str,
) -> sqlx::Result<bool> {
  let user: Option<UserRow> = sqlx::query_as(
    r#"SELECT u.*
    FROM users u
    WHERE u.application_id = $1 AND u.username = $2
    LIMIT 1;"#,
  )
  .bind(application_id)
  .bind(username)
  .fetch_optional(&mut **transaction)
  .await?;
  Ok(user.is_some())
}

pub struct UpdateUser {
  pub username: Option<String>,
  pub active: Option<i32>,
}

pub async fn update_user(
  pool: &sqlx::AnyPool,
  application_id: i64,
  user_id: i64,
  params: UpdateUser,
) -> sqlx::Result<Option<UserRow>> {
  sqlx::query_as(
    r#"UPDATE users SET "username" = COALESCE($2, "username"), "active" = COALESCE($3, "active") WHERE application_id = $1 AND id = $2 RETURNING *;"#,
  )
  .bind(application_id)
  .bind(user_id)
  .bind(params.username)
  .bind(params.active)
  .fetch_optional(pool)
  .await
}

pub async fn delete_user(
  pool: &sqlx::AnyPool,
  application_id: i64,
  user_id: i64,
) -> sqlx::Result<Option<UserRow>> {
  sqlx::query_as(r#"DELETE FROM users WHERE application_id = $1 AND id = $2 RETURNING *;"#)
    .bind(application_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}
