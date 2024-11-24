#[derive(sqlx::FromRow)]
pub struct UserPhoneNumberRow {
  pub id: i64,
  pub user_id: i64,
  pub primary: i32,
  pub confirmed: i32,
  pub phone_number: String,
  pub created_at: i64,
  pub updated_at: i64,
}

impl UserPhoneNumberRow {
  pub fn is_confirmed(&self) -> bool {
    self.confirmed != 0
  }
  pub fn is_primary(&self) -> bool {
    self.primary != 0
  }
}

pub async fn get_user_phone_numbers_by_user_id(
  pool: &sqlx::AnyPool,
  user_id: i64,
) -> sqlx::Result<Vec<UserPhoneNumberRow>> {
  sqlx::query_as(
    r#"SELECT upn.*
    FROM user_phone_numbers upn
    WHERE upn.user_id = $1;"#,
  )
  .bind(user_id)
  .fetch_all(pool)
  .await
}
