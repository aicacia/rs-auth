#[derive(sqlx::FromRow)]
pub struct UserEmailRow {
  pub id: i64,
  pub user_id: i64,
  pub primary: i32,
  pub confirmed: i32,
  pub email: String,
  pub created_at: i64,
  pub updated_at: i64,
}

impl UserEmailRow {
  pub fn is_confirmed(&self) -> bool {
    self.confirmed != 0
  }
  pub fn is_primary(&self) -> bool {
    self.primary != 0
  }
}

pub async fn get_user_emails_by_user_id(
  pool: &sqlx::AnyPool,
  user_id: i64,
) -> sqlx::Result<Vec<UserEmailRow>> {
  sqlx::query_as(
    r#"SELECT ue.*
    FROM user_emails ue
    WHERE ue.user_id = $1;"#,
  )
  .bind(user_id)
  .fetch_all(pool)
  .await
}
