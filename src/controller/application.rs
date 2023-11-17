use actix_web::{
  get,
  web::{scope, Data, Path, ServiceConfig},
  HttpResponse, Responder,
};
use actix_web_validator::Query;
use sqlx::{Pool, Postgres};

use crate::{
  middleware::{admin::AdminAuthorization, auth::Authorization},
  model::{
    application::{Application, ApplicationRow, PaginationApplication, PaginationApplicationQuery},
    error::Errors,
    user::UserRow,
  },
  service::{
    application::{get_application_by_id, get_applications},
    user::user_has_permissions,
  },
};

#[utoipa::path(
  context_path = "/applications",
  responses(
    (status = 200, description = "Get all applications", body = PaginationApplication),
    (status = 500, body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
#[get("")]
pub async fn index(
  pool: Data<Pool<Postgres>>,
  user: UserRow,
  application: ApplicationRow,
  query: Query<PaginationApplicationQuery>,
) -> impl Responder {
  match user_has_permissions(pool.as_ref(), user.id, application.id, "admin").await {
    Ok(true) => {}
    Ok(false) => return HttpResponse::Forbidden().finish(),
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(Errors::internal_error());
    }
  };
  let page_size = query.page_size.unwrap_or(20);
  let applications = match get_applications(pool.as_ref(), query.page.unwrap_or(0), page_size).await
  {
    Ok(u) => u,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(Errors::internal_error());
    }
  };
  let applications_response: Vec<Application> =
    applications.into_iter().map(Into::into).collect::<Vec<_>>();
  HttpResponse::Ok().json(PaginationApplication {
    has_more: applications_response.len() == page_size as usize,
    data: applications_response,
  })
}

#[utoipa::path(
  context_path = "/applications",
  responses(
    (status = 200, description = "Get an application", body = Application),
    (status = 500, body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
#[get("/{application_id}")]
pub async fn show(
  pool: Data<Pool<Postgres>>,
  path: Path<i32>,
  user: UserRow,
  application: ApplicationRow,
) -> impl Responder {
  match user_has_permissions(pool.as_ref(), user.id, application.id, "admin").await {
    Ok(true) => {}
    Ok(false) => return HttpResponse::Forbidden().finish(),
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(Errors::internal_error());
    }
  };
  let application_id = path.into_inner();
  let application = match get_application_by_id(pool.as_ref(), application_id).await {
    Ok(Some(a)) => a,
    Ok(None) => return HttpResponse::NotFound().finish(),
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::BadRequest().json(Errors::internal_error());
    }
  };
  let applications_response: Application = application.into();
  HttpResponse::Ok().json(applications_response)
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config.service(
      scope("/applications")
        .wrap(AdminAuthorization)
        .wrap(Authorization)
        .service(index)
        .service(show),
    );
  }
}
