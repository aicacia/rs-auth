#[derive(sqlx::FromRow)]
pub struct UserTOTPRow {
  pub user_id: i64,
  pub active: i32,
  pub algorithm: String,
  pub digits: i64,
  pub step: i64,
  pub secret: String,
  pub created_at: i64,
  pub updated_at: i64,
}

pub async fn get_user_totp_by_user_id(
  pool: &sqlx::AnyPool,
  user_id: i64,
) -> sqlx::Result<Option<UserTOTPRow>> {
  sqlx::query_as(
    r#"SELECT ut.*
    FROM user_totps ut
    WHERE ut.user_id = $1 
    LIMIT 1;"#,
  )
  .bind(user_id)
  .fetch_optional(pool)
  .await
}

#[derive(Default)]
pub struct CreateUserTOTPRow {
  pub algorithm: String,
  pub digits: i64,
  pub step: i64,
  pub secret: String,
}

pub async fn create_user_totp(
  pool: &sqlx::AnyPool,
  user_id: i64,
  params: CreateUserTOTPRow,
) -> sqlx::Result<UserTOTPRow> {
  sqlx::query_as(
    r#"INSERT INTO user_totps (user_id, algorithm, digits, step, secret)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING *;"#,
  )
  .bind(user_id)
  .bind(params.algorithm)
  .bind(params.digits)
  .bind(params.step)
  .bind(params.secret)
  .fetch_one(pool)
  .await
}

#[derive(Default)]
pub struct UpdateUserTOTPRow {
  pub algorithm: Option<String>,
  pub digits: Option<i64>,
  pub step: Option<i64>,
  pub secret: Option<String>,
}

pub async fn update_user_totp(
  pool: &sqlx::AnyPool,
  user_id: i64,
  params: UpdateUserTOTPRow,
) -> sqlx::Result<Option<UserTOTPRow>> {
  sqlx::query_as(
    r#"UPDATE user_totps SET 
      algorithm = COALESCE($2, algorithm),
      digits = COALESCE($3, digits),
      step = COALESCE($4, step),
      secret = COALESCE($5, secret)
    WHERE user_id = $1
    RETURNING *;"#,
  )
  .bind(user_id)
  .bind(params.algorithm)
  .bind(params.digits)
  .bind(params.step)
  .bind(params.secret)
  .fetch_optional(pool)
  .await
}
