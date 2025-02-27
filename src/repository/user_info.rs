use super::user::from_users_query;

#[derive(Debug, sqlx::FromRow)]
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
  pub birthdate: Option<i64>,
  pub zone_info: Option<String>,
  pub locale: Option<String>,
  pub address: Option<String>,
  pub updated_at: i64,
  pub created_at: i64,
}

pub async fn get_users_infos(
  pool: &sqlx::AnyPool,
  application_id: i64,
  limit: Option<usize>,
  offset: Option<usize>,
) -> sqlx::Result<Vec<UserInfoRow>> {
  let mut qb =
    sqlx::QueryBuilder::new("SELECT ui.* FROM user_infos ui WHERE ui.user_id IN (SELECT u.id");
  from_users_query(&mut qb, application_id, limit, offset);
  qb.push(")");
  qb.build_query_as().fetch_all(pool).await
}

pub async fn get_user_info_by_user_id(
  pool: &sqlx::AnyPool,
  application_id: i64,
  user_id: i64,
) -> sqlx::Result<Option<UserInfoRow>> {
  sqlx::query_as(
    r#"SELECT ui.*
    FROM user_infos ui
    JOIN users u on ui.user_id = u.id
    WHERE u.application_id = $1 AND ui.user_id = $2
    LIMIT 1;"#,
  )
  .bind(application_id)
  .bind(user_id)
  .fetch_optional(pool)
  .await
}

#[derive(Default)]
pub struct UserInfoUpdate {
  pub name: Option<String>,
  pub given_name: Option<String>,
  pub family_name: Option<String>,
  pub middle_name: Option<String>,
  pub nickname: Option<String>,
  pub profile_picture: Option<String>,
  pub website: Option<String>,
  pub gender: Option<String>,
  pub birthdate: Option<i64>,
  pub zone_info: Option<String>,
  pub locale: Option<String>,
  pub address: Option<String>,
}

pub async fn update_user_info(
  pool: &sqlx::AnyPool,
  user_id: i64,
  updates: UserInfoUpdate,
) -> sqlx::Result<UserInfoRow> {
  sqlx::query_as(
    r#"UPDATE user_infos SET
      name = COALESCE($2, name),
      given_name = COALESCE($3, given_name),
      family_name = COALESCE($4, family_name),
      middle_name = COALESCE($5, middle_name),
      nickname = COALESCE($6, nickname),
      profile_picture = COALESCE($7, profile_picture),
      website = COALESCE($8, website),
      gender = COALESCE($9, gender),
      birthdate = COALESCE($10, birthdate),
      zone_info = COALESCE($11, zone_info),
      locale = COALESCE($12, locale),
      address = COALESCE($13, address),
      updated_at = $14
    WHERE user_id = $1
    RETURNING *;"#,
  )
  .bind(user_id)
  .bind(updates.name)
  .bind(updates.given_name)
  .bind(updates.family_name)
  .bind(updates.middle_name)
  .bind(updates.nickname)
  .bind(updates.profile_picture)
  .bind(updates.website)
  .bind(updates.gender)
  .bind(updates.birthdate)
  .bind(updates.zone_info)
  .bind(updates.locale)
  .bind(updates.address)
  .bind(chrono::Utc::now().timestamp())
  .fetch_one(pool)
  .await
}
