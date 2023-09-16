use anyhow::Result;
use sqlx::{Connection, Pool, Postgres};
use uuid::Uuid;

use crate::model::user::{Email, User};

pub async fn get_user_by_id(pool: &Pool<Postgres>, user_id: i32) -> Result<User> {
  let user = sqlx::query_as!(
    User,
    r#"SELECT
      u.id, u.email_id, u.username, u.encrypted_password, u.reset_password_token, u.created_at, u.updated_at
    FROM users u LEFT JOIN emails e ON e.id = u.email_id
    WHERE u.id = $1
    LIMIT 1;"#,
    user_id
  )
  .fetch_one(pool)
  .await?;
  Ok(user)
}

pub async fn get_user_emails(pool: &Pool<Postgres>, user_id: i32) -> Result<Vec<Email>> {
  let emails = sqlx::query_as!(
    Email,
    r#"SELECT
      e.id, e.user_id, e.email, e.confirmed, e.confirmation_token, e.created_at, e.updated_at
    FROM emails e
    WHERE e.user_id = $1;"#,
    user_id
  )
  .fetch_all(pool)
  .await?;
  Ok(emails)
}

pub async fn get_user_by_username_or_email(
  pool: &Pool<Postgres>,
  username_or_email: &str,
) -> Result<User> {
  let user = sqlx::query_as!(
    User,
    r#"SELECT
      u.id, u.email_id, u.username, u.encrypted_password, u.reset_password_token, u.created_at, u.updated_at
    FROM users u
    LEFT JOIN emails e ON e.id=u.email_id
    WHERE e.email = $1 OR u.username = $1
    LIMIT 1;"#,
    username_or_email
  )
  .fetch_one(pool)
  .await?;
  Ok(user)
}

pub struct CreateUser {
  pub username: String,
  pub email: Option<String>,
  pub encrypted_password: String,
}

pub async fn create_user(
  pool: &Pool<Postgres>,
  create_user: CreateUser,
) -> Result<(User, Option<Email>)> {
  let mut conn = pool.acquire().await?;

  let (user, email) = conn.transaction::<_, _, sqlx::Error>(|tx| Box::pin(async move {
        let mut user: User = sqlx::query_as!(
            User,
            "INSERT INTO users (username, encrypted_password) VALUES ($1, $2) RETURNING *;",
            &create_user.username,
            &create_user.encrypted_password
        )
        .fetch_one(&mut **tx)
        .await?;
        let email = if let Some(email_str) = create_user.email.as_ref() {
            let confirmation_token = Uuid::new_v4();
            let email = sqlx::query_as!(
              Email,
                r#"INSERT INTO emails ("user_id", "email", "confirmation_token") VALUES ($1, $2, $3) RETURNING *;"#,
                user.id,
                email_str,
                &confirmation_token
            )
            .fetch_one(&mut **tx)
            .await?;

            sqlx::query!(
              r#"UPDATE users SET "email_id" = $2 WHERE id = $1;"#,
              user.id,
              email.id
            ).execute(&mut **tx).await?;

            Some(email)
        } else {
          None
        };
        if let Some(e) = email.as_ref() {
          user.email_id = Some(e.id);
        }
        Ok((user, email))
    })).await?;

  Ok((user, email))
}

pub async fn request_user_password_reset(
  pool: &Pool<Postgres>,
  email: &str,
) -> Result<(User, Uuid)> {
  let reset_password_token = Uuid::new_v4();
  let user = sqlx::query_as!(
        User,
        "UPDATE users u SET reset_password_token=$1 FROM emails e WHERE e.user_id=u.id AND e.email=$2 RETURNING u.*;",
        &reset_password_token,
        &email
    )
    .fetch_one(pool)
    .await?;
  Ok((user, reset_password_token))
}

pub async fn get_user_by_reset_token(
  pool: &Pool<Postgres>,
  reset_password_token: &Uuid,
) -> Result<User> {
  let user = sqlx::query_as!(
        User,
        r#"SELECT
          u.id, u.email_id, u.username, u.encrypted_password, u.reset_password_token, u.created_at, u.updated_at
        FROM users u
        WHERE u.reset_password_token=$1
        LIMIT 1;"#,
        &reset_password_token
    )
    .fetch_one(pool)
    .await?;
  Ok(user)
}

pub async fn reset_user_password(
  pool: &Pool<Postgres>,
  user_id: i32,
  encrypted_password: &str,
) -> Result<bool> {
  let result = sqlx::query_as!(
    User,
    "UPDATE users SET encrypted_password = $1, reset_password_token = null WHERE id = $2",
    &encrypted_password,
    user_id
  )
  .execute(pool)
  .await?;
  Ok(result.rows_affected() > 0)
}

pub async fn confirm_user_email(
  pool: &Pool<Postgres>,
  user_id: i32,
  confirmation_token: &Uuid,
) -> Result<bool> {
  let result = sqlx::query!(
      "UPDATE emails SET confirmed=true, confirmation_token=NULL WHERE user_id = $1 AND confirmation_token = $2;",
      user_id,
      confirmation_token
  )
  .execute(pool)
  .await?;
  Ok(result.rows_affected() > 0)
}

pub async fn set_user_primary_email(
  pool: &Pool<Postgres>,
  user_id: i32,
  email_id: i32,
) -> Result<bool> {
  let result = sqlx::query!(
    "UPDATE users u SET email_id=$2 FROM emails e WHERE u.id=$1 AND e.id=$2 AND u.id = e.user_id;",
    user_id,
    email_id
  )
  .execute(pool)
  .await?;
  Ok(result.rows_affected() > 0)
}
