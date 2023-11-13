use anyhow::Result;
use sqlx::{Connection, Pool, Postgres};
use uuid::Uuid;

use crate::{
  core::mail::send_support_mail,
  model::{
    application::ApplicationRow,
    user::{EmailRow, UserRow},
  },
};

pub async fn get_user_by_id(pool: &Pool<Postgres>, user_id: i32) -> Result<UserRow> {
  let user = sqlx::query_as!(
    UserRow,
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

pub async fn get_user_emails(pool: &Pool<Postgres>, user_id: i32) -> Result<Vec<EmailRow>> {
  let emails = sqlx::query_as!(
    EmailRow,
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
) -> Result<UserRow> {
  let user = sqlx::query_as!(
    UserRow,
    r#"SELECT
      u.id, u.email_id, u.username, u.encrypted_password, u.reset_password_token, u.created_at, u.updated_at
    FROM users u
    LEFT JOIN emails e ON e.user_id=u.id
    WHERE e.email = $1 OR u.username = $1
    LIMIT 1;"#,
    username_or_email
  )
  .fetch_one(pool)
  .await?;
  Ok(user)
}

pub async fn user_username_taken(pool: &Pool<Postgres>, username: &str) -> Result<bool> {
  Ok(
    sqlx::query!(
      r#"SELECT u.id
      FROM users u
      WHERE u.username = $1
      LIMIT 1;"#,
      username
    )
    .fetch_optional(pool)
    .await?
    .map_or(false, |_| true),
  )
}

pub async fn user_email_taken(pool: &Pool<Postgres>, email: &str) -> Result<bool> {
  Ok(
    sqlx::query!(
      r#"SELECT u.id
      FROM users u
      JOIN emails e ON e.user_id = u.id
      WHERE e.email = $1
      LIMIT 1;"#,
      email
    )
    .fetch_optional(pool)
    .await?
    .map_or(false, |_| true),
  )
}

pub struct CreateUser {
  pub username: String,
  pub email: Option<String>,
  pub encrypted_password: String,
  pub send_confirmation_token: bool,
}

pub async fn create_user(
  pool: &Pool<Postgres>,
  application_id: i32,
  create_user: CreateUser,
) -> Result<(UserRow, Option<EmailRow>)> {
  let mut conn = pool.acquire().await?;

  let (user, email) = conn.transaction::<_, _, sqlx::Error>(|tx| Box::pin(async move {
        let mut user: UserRow = sqlx::query_as!(
            UserRow,
            "INSERT INTO users (username, encrypted_password) VALUES ($1, $2)
            RETURNING id, email_id, username, encrypted_password, reset_password_token, created_at, updated_at;",
            &create_user.username,
            &create_user.encrypted_password
        )
        .fetch_one(&mut **tx)
        .await?;

        let email = if let Some(email_str) = create_user.email.as_ref() {
            let confirmation_token = Uuid::new_v4();
            let email = sqlx::query_as!(
              EmailRow,
                r#"INSERT INTO emails ("user_id", "email", "confirmation_token") VALUES ($1, $2, $3)
                RETURNING "id", "user_id", "email", "confirmed", "confirmation_token", "created_at", "updated_at";"#,
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

  if create_user.send_confirmation_token {
    if let Some(email) = email.as_ref() {
      if let Some(confirmation_token) = email.confirmation_token {
        send_support_mail(
          pool.clone(),
          application_id,
          user.username.to_owned(),
          email.email.to_owned(),
          "Confirmation Token".to_owned(),
          format!(
            r#"<h1>Welcome!</h1>
          <p>Your confirmation token is: <code>{}</code></p>"#,
            confirmation_token
          ),
        );
      }
    }
  }

  Ok((user, email))
}

pub async fn add_user_application(
  pool: &Pool<Postgres>,
  user_id: i32,
  application_id: i32,
) -> Result<()> {
  sqlx::query!(
    "INSERT INTO application_users (application_id, user_id) VALUES ($1, $2);",
    application_id,
    user_id
  )
  .execute(pool)
  .await?;
  Ok(())
}

pub async fn get_user_applications(
  pool: &Pool<Postgres>,
  user_id: i32,
) -> Result<Vec<ApplicationRow>> {
  Ok(
    sqlx::query_as!(
      ApplicationRow,
      r#"SELECT
        a.id, a.name, a.uri, a.created_at, a.updated_at
      FROM application_users au
        JOIN applications a ON a.id=au.application_id
      WHERE au.user_id=$1;"#,
      user_id
    )
    .fetch_all(pool)
    .await?,
  )
}

pub async fn request_user_password_reset(
  pool: &Pool<Postgres>,
  application_id: i32,
  email: &str,
) -> Result<(UserRow, Uuid)> {
  let reset_password_token = Uuid::new_v4();
  let user = sqlx::query_as!(
    UserRow,
    "UPDATE users u SET reset_password_token=$1 FROM emails e WHERE e.user_id=u.id AND e.email=$2
        RETURNING u.id, u.email_id, u.username, u.encrypted_password, u.reset_password_token, u.created_at, u.updated_at;",
    &reset_password_token,
    &email
  )
  .fetch_one(pool)
  .await?;

  send_support_mail(
    pool.clone(),
    application_id,
    user.username.to_owned(),
    email.to_owned(),
    "Reset Password Request".to_owned(),
    format!(
      r#"<h1>A Request to reset your password was made.</h1>
    <p>Your password reset token is: <code>{}</code></p>"#,
      reset_password_token
    ),
  );

  Ok((user, reset_password_token))
}

pub async fn get_user_by_reset_token(
  pool: &Pool<Postgres>,
  reset_password_token: &Uuid,
) -> Result<UserRow> {
  let user = sqlx::query_as!(
        UserRow,
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
    UserRow,
    "UPDATE users SET encrypted_password = $1, reset_password_token = null WHERE id = $2",
    &encrypted_password,
    user_id
  )
  .execute(pool)
  .await?;
  Ok(result.rows_affected() > 0)
}

pub async fn set_user_email_confirmation_token(
  pool: &Pool<Postgres>,
  user_id: i32,
  email_id: i32,
  confirmation_token: &Uuid,
) -> Result<Option<EmailRow>> {
  Ok(sqlx::query_as!(
    EmailRow,
    "UPDATE emails SET confirmation_token=$1 WHERE user_id=$2 AND id=$3 RETURNING id, user_id, email, confirmed, confirmation_token, created_at, updated_at;",
    confirmation_token,
    user_id,
    email_id,
  )
  .fetch_optional(pool)
  .await?)
}

pub async fn confirm_user_email(pool: &Pool<Postgres>, confirmation_token: &Uuid) -> Result<bool> {
  let result = sqlx::query!(
    "UPDATE emails SET confirmed=true, confirmation_token=NULL WHERE confirmation_token = $1;",
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

pub async fn user_has_application(
  pool: &Pool<Postgres>,
  user_id: i32,
  application_id: i32,
) -> Result<bool> {
  let application_user = sqlx::query!(
    r#"SELECT application_id, user_id FROM application_users WHERE application_id=$1 AND user_id=$2 LIMIT 1;"#,
    application_id,
    user_id
  )
  .fetch_optional(pool)
  .await?;
  Ok(application_user.is_some())
}

pub async fn change_user_username(
  pool: &Pool<Postgres>,
  user_id: i32,
  username: &str,
) -> sqlx::Result<UserRow> {
  sqlx::query_as!(
      UserRow,
      r#"UPDATE users SET username=$1 WHERE id=$2 RETURNING id, email_id, username, encrypted_password, reset_password_token, created_at, updated_at;"#,
      username,
      user_id
    )
    .fetch_one(pool)
    .await
}
