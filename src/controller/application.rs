use actix_web::{
  delete, get, patch, post,
  web::{scope, Data, Path, ServiceConfig},
  HttpResponse, Responder,
};
use actix_web_validator::{Json, Query};
use sqlx::{Pool, Postgres};

use crate::{
  middleware::{admin::AdminAuthorization, auth::Authorization},
  model::{
    application::{
      Application, ApplicationConfig, ApplicationWithSecret, CreateApplicationRequest,
      PaginationApplication, PaginationApplicationQuery, UpdateApplicationConfigRequest,
      UpdateApplicationRequest,
    },
    error::Errors,
  },
  service::application::{
    create_application, delete_application, get_application_by_id, get_application_configs,
    get_applications, reset_application_secret, set_application_config, update_application,
  },
};

#[utoipa::path(
  context_path = "/applications",
  params(PaginationApplicationQuery),
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
  query: Query<PaginationApplicationQuery>,
) -> impl Responder {
  let page_size = query.page_size.unwrap_or(20);
  let applications = match get_applications(pool.as_ref(), query.page.unwrap_or(0), page_size).await
  {
    Ok(u) => u,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
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
pub async fn show(pool: Data<Pool<Postgres>>, path: Path<i32>) -> impl Responder {
  let application_id = path.into_inner();
  let application = match get_application_by_id(pool.as_ref(), application_id).await {
    Ok(Some(a)) => a,
    Ok(None) => return HttpResponse::NotFound().json(Errors::not_found()),
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  let applications_response: Application = application.into();
  HttpResponse::Ok().json(applications_response)
}

#[utoipa::path(
  context_path = "/applications",
  request_body = CreateApplicationRequest,
  responses(
    (status = 200, description = "Update an application", body = Application),
    (status = 500, body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
#[post("")]
pub async fn create(
  pool: Data<Pool<Postgres>>,
  body: Json<CreateApplicationRequest>,
) -> impl Responder {
  let application = match create_application(pool.as_ref(), &body.description, &body.uri).await {
    Ok(a) => a,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  let applications_response: ApplicationWithSecret = application.into();
  HttpResponse::Ok().json(applications_response)
}

#[utoipa::path(
  context_path = "/applications",
  request_body = UpdateApplicationRequest,
  responses(
    (status = 200, description = "Update an application", body = Application),
    (status = 500, body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
#[patch("/{application_id}")]
pub async fn update(
  pool: Data<Pool<Postgres>>,
  path: Path<i32>,
  body: Json<UpdateApplicationRequest>,
) -> impl Responder {
  let application_id = path.into_inner();
  let application = match update_application(
    pool.as_ref(),
    application_id,
    body.description.as_ref(),
    body.uri.as_ref(),
  )
  .await
  {
    Ok(Some(a)) => a,
    Ok(None) => return HttpResponse::NotFound().json(Errors::not_found()),
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  let applications_response: Application = application.into();
  HttpResponse::Ok().json(applications_response)
}

#[utoipa::path(
  context_path = "/applications",
  responses(
    (status = 200, description = "Resets an application's secret", body = ApplicationWithSecret),
    (status = 500, body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
#[patch("/{application_id}/reset-secret")]
pub async fn reset_secret(pool: Data<Pool<Postgres>>, path: Path<i32>) -> impl Responder {
  let application_id = path.into_inner();
  let application = match reset_application_secret(pool.as_ref(), application_id).await {
    Ok(Some(a)) => a,
    Ok(None) => return HttpResponse::NotFound().json(Errors::not_found()),
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  let applications_response: ApplicationWithSecret = application.into();
  HttpResponse::Ok().json(applications_response)
}

#[utoipa::path(
  context_path = "/applications",
  responses(
    (status = 204, description = "Delete an application"),
    (status = 500, body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
#[delete("/{application_id}")]
pub async fn remove(pool: Data<Pool<Postgres>>, path: Path<i32>) -> impl Responder {
  let application_id = path.into_inner();
  match delete_application(pool.as_ref(), application_id).await {
    Ok(Some(_)) => {}
    Ok(None) => return HttpResponse::NotFound().json(Errors::not_found()),
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  HttpResponse::NoContent().finish()
}

#[utoipa::path(
  context_path = "/applications",
  responses(
    (status = 200, description = "Get an application config", body = Vec<ApplicationConfig>),
    (status = 500, body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
#[get("/{application_id}/config")]
pub async fn config(pool: Data<Pool<Postgres>>, path: Path<i32>) -> impl Responder {
  let application_id = path.into_inner();
  let application_configs = match get_application_configs(pool.as_ref(), application_id).await {
    Ok(ac) => ac,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  let application_configs_response: Vec<ApplicationConfig> = application_configs
    .into_iter()
    .map(Into::into)
    .collect::<Vec<_>>();
  HttpResponse::Ok().json(application_configs_response)
}

#[utoipa::path(
  context_path = "/applications",
  request_body = UpdateApplicationConfigRequest,
  responses(
    (status = 204, description = "Update an application config key"),
    (status = 500, body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
#[patch("/{application_id}/config")]
pub async fn update_config(
  pool: Data<Pool<Postgres>>,
  path: Path<i32>,
  body: Json<UpdateApplicationConfigRequest>,
) -> impl Responder {
  let application_id = path.into_inner();
  match set_application_config(pool.as_ref(), application_id, &body.key, &body.value).await {
    Ok(_) => {}
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  HttpResponse::NoContent().finish()
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |service_config: &mut ServiceConfig| {
    service_config.service(
      scope("/applications")
        .wrap(AdminAuthorization)
        .wrap(Authorization)
        .service(index)
        .service(show)
        .service(create)
        .service(update)
        .service(reset_secret)
        .service(config)
        .service(update_config)
        .service(remove),
    );
  }
}
