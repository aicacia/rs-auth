#[derive(sqlx::FromRow)]
pub struct ApplicationRow {
  pub id: i64,
  pub name: String,
  pub updated_at: i64,
  pub created_at: i64,
}

pub async fn get_applications(
  pool: &sqlx::AnyPool,
  limit: Option<usize>,
  offset: Option<usize>,
) -> sqlx::Result<Vec<ApplicationRow>> {
  let mut qb = sqlx::QueryBuilder::new("SELECT a.* FROM applications a");
  if let Some(limit) = limit {
    qb.push(" LIMIT ").push_bind(limit as i64);
  }
  if let Some(offset) = offset {
    qb.push(" OFFSET ").push_bind(offset as i64);
  }
  qb.build_query_as().fetch_all(pool).await
}

pub async fn get_application_by_id(
  pool: &sqlx::AnyPool,
  application_id: i64,
) -> sqlx::Result<Option<ApplicationRow>> {
  sqlx::query_as(
    r#"SELECT a.*
    FROM applications a
    WHERE a.id = $1
    LIMIT 1;"#,
  )
  .bind(application_id)
  .fetch_optional(pool)
  .await
}

pub struct CreateApplication {
  pub name: String,
}

pub async fn create_application(
  pool: &sqlx::AnyPool,
  params: CreateApplication,
) -> sqlx::Result<ApplicationRow> {
  sqlx::query_as(r#"INSERT INTO applications (name) VALUES ($1) RETURNING *;"#)
    .bind(params.name)
    .fetch_one(pool)
    .await
}

pub struct UpdateApplication {
  pub name: Option<String>,
}

pub async fn update_application(
  pool: &sqlx::AnyPool,
  application_id: i64,
  params: UpdateApplication,
) -> sqlx::Result<Option<ApplicationRow>> {
  sqlx::query_as(
    r#"UPDATE applications SET
      name = COALESCE($2, name)
      updated_at = $3
    WHERE id = $1
    RETURNING *;"#,
  )
  .bind(application_id)
  .bind(params.name)
  .bind(chrono::Utc::now().timestamp())
  .fetch_optional(pool)
  .await
}

pub async fn delete_application(
  pool: &sqlx::AnyPool,
  application_id: i64,
) -> sqlx::Result<Option<ApplicationRow>> {
  sqlx::query_as(
    r#"DELETE FROM applications
    WHERE id = $1
    RETURNING *;"#,
  )
  .bind(application_id)
  .fetch_optional(pool)
  .await
}
