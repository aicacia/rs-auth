use sqlx::{AnyPool, Result};

#[derive(sqlx::FromRow)]
pub struct UserInfoRow {
  pub user_id: i64,
  pub name: Option<String>,
  pub given_name: Option<String>,
  pub family_name: Option<String>,
  pub middle_name: Option<String>,
  pub nickname: Option<String>,
  pub profile_picture: Option<String>,
  pub website: Option<String>,
  pub gender: Option<String>,
  pub birthdate: Option<String>,
  pub zone_info: Option<String>,
  pub locale: Option<String>,
  pub address: Option<String>,
  pub created_at: i64,
  pub updated_at: i64,
}

pub async fn get_user_info_by_user_id(pool: &AnyPool, user_id: i64) -> Result<Option<UserInfoRow>> {
  let user_info = sqlx::query_as(
    r#"SELECT ui.*
    FROM user_infos ui
    WHERE ui.user_id = $1 
    LIMIT 1;"#,
  )
  .bind(user_id)
  .fetch_optional(pool)
  .await?;
  Ok(user_info)
}
