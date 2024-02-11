use actix_web::{
  get,
  web::{Data, Path, ServiceConfig},
  HttpResponse, Responder,
};
use actix_web_validator::Query;
use sqlx::{Pool, Postgres};

use crate::{
  core::{
    config::get_config,
    jwt::{encode_jwt, parse_jwt},
  },
  model::{
    error::Errors,
    oauth2::{AuthorizeQuery, OAuth2Application, OAuth2CodeClaims},
  },
  service::application::{get_application_by_id, get_application_jwt_secret, get_application_uri},
};

#[utoipa::path(
  params(AuthorizeQuery),
  responses(
    (status = 302, description = "Redirected to url"),
    (status = 500, body = Errors),
  )
)]
#[get("/oauth2/authorize")]
pub async fn authorize(pool: Data<Pool<Postgres>>, query: Query<AuthorizeQuery>) -> impl Responder {
  let config = get_config();
  let admin_uri = get_application_uri(pool.as_ref(), config.admin_application_id).await;

  let now_in_seconds = chrono::Utc::now().timestamp() as usize;
  let secret = get_application_jwt_secret(pool.as_ref(), config.admin_application_id).await;
  let iss = config
    .server
    .uri
    .as_ref()
    .map(String::as_str)
    .unwrap_or("Auth");
  let code_jwt = match encode_jwt(
    &OAuth2CodeClaims::new(
      query.client_id.clone(),
      query.redirect_uri.clone(),
      query.state.clone(),
      query.scope.clone().unwrap_or_default(),
      now_in_seconds,
      120,
      iss,
    ),
    &secret,
  ) {
    Ok(jwt) => jwt,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };

  let uri = format!(
    "{}/oauth2?code={}",
    admin_uri,
    urlencoding::encode(&code_jwt),
  );

  HttpResponse::TemporaryRedirect()
    .append_header(("Location", uri))
    .finish()
}

#[utoipa::path(
  responses(
    (status = 200, description = "OAuth2 authorize information", body = OAuth2Application),
    (status = 500, body = Errors),
  )
)]
#[get("/oauth2/{code}/application")]
pub async fn application(pool: Data<Pool<Postgres>>, path: Path<String>) -> impl Responder {
  let config = get_config();
  let code = path.into_inner();
  let secret = get_application_jwt_secret(pool.as_ref(), config.admin_application_id).await;
  let claims = match parse_jwt::<OAuth2CodeClaims>(&code, &secret) {
    Ok(jwt) => jwt.claims,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::Unauthorized().json(Errors::unauthorized());
    }
  };
  let application = match get_application_by_id(pool.as_ref(), claims.client_id).await {
    Ok(Some(a)) => a,
    Ok(None) => {
      return HttpResponse::NotFound().json(Errors::not_found());
    }
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::Unauthorized().json(Errors::unauthorized());
    }
  };
  let oauth2_application: OAuth2Application =
    (application, claims.scope, claims.redirect_uri, claims.state).into();
  HttpResponse::Ok().json(oauth2_application)
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config.service(authorize).service(application);
  }
}
