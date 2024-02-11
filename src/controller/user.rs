use actix_web::{
  delete, get, post, put,
  web::{scope, Data, Path, Query, ServiceConfig},
  HttpResponse, Responder,
};
use actix_web_validator::Json;
use futures::try_join;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
  core::{encryption::encrypt_password, mail::send_support_mail},
  middleware::{admin::AdminAuthorization, admin_app::AdminAppAuthorization, auth::Authorization},
  model::{
    application::{Application, ApplicationRow, PaginationApplicationQuery},
    error::Errors,
    user::{
      ChangeUsernameRequest, CreateUserEmailRequest, Email, PaginationUser, PaginationUserQuery,
      ResetUserPasswordRequest, User, UserRow,
    },
  },
  service::user::{
    change_user_username, create_user_email, delete_user_email, get_user_applications,
    get_user_emails, get_user_permissions, get_users, reset_user_password,
    set_user_email_confirmation_token, set_user_primary_email,
  },
};

#[utoipa::path(
  context_path = "/users",
  responses(
      (status = 200, description = "Get current user", body = User),
      (status = 500, body = Errors),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[get("/current")]
pub async fn current(
  pool: Data<Pool<Postgres>>,
  user: UserRow,
  application: ApplicationRow,
) -> impl Responder {
  let (emails, permissions) = match try_join!(
    get_user_emails(pool.as_ref(), user.id),
    get_user_permissions(pool.as_ref(), user.id, application.id),
  ) {
    Ok(r) => r,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  let user_response: User = (user, emails, permissions).into();
  HttpResponse::Ok().json(user_response)
}

#[utoipa::path(
  context_path = "/users",
  request_body = CreateUserEmailRequest,
  responses(
      (status = 201, description = "Creates email", body = Email),
      (status = 400, body = Errors),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[post("/emails")]
pub async fn create_email(
  user: UserRow,
  body: Json<CreateUserEmailRequest>,
  pool: Data<Pool<Postgres>>,
) -> impl Responder {
  let email = match create_user_email(pool.as_ref(), user.id, &body.email).await {
    Ok(e) => e,
    Err(e) => {
      log::error!("{}", e);
      match &e {
        sqlx::Error::Database(e) => {
          if let Some(_) = e.constraint() {
            return HttpResponse::BadRequest().json(Errors::from("taken"));
          }
        }
        _ => (),
      }
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  let email_response: Email = email.into();
  HttpResponse::Created().json(email_response)
}

#[utoipa::path(
  context_path = "/users",
  responses(
    (status = 204, description = "User's email deleted"),
    (status = 400, body = Errors),
  ),
  security(
    ("Authorization" = [])
  )
)]
#[delete("/emails/{email_id}")]
pub async fn delete_email(
  user: UserRow,
  path: Path<i32>,
  pool: Data<Pool<Postgres>>,
) -> impl Responder {
  let email_id = path.into_inner();
  match delete_user_email(pool.as_ref(), user.id, email_id).await {
    Ok(_) => {}
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  }
  HttpResponse::NoContent().finish()
}

#[utoipa::path(
  context_path = "/users",
  responses(
      (status = 204, description = "Sets email as primary"),
      (status = 400, body = Errors),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[put("/emails/{email_id}/set-primary-email")]
pub async fn set_primary_email(
  user: UserRow,
  path: Path<i32>,
  pool: Data<Pool<Postgres>>,
) -> impl Responder {
  let email_id = path.into_inner();
  match set_user_primary_email(pool.as_ref(), user.id, email_id).await {
    Ok(true) => (),
    Ok(false) => {
      return HttpResponse::BadRequest().json(Errors::new().error("email_id", "invalid_email"));
    }
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  HttpResponse::NoContent().finish()
}

#[utoipa::path(
  context_path = "/users",
  responses(
      (status = 204, description = "Sends confirmation email"),
      (status = 400, body = Errors),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[put("/send-confirmation/{email_id}")]
pub async fn send_confirmation_email(
  application: ApplicationRow,
  user: UserRow,
  path: Path<i32>,
  pool: Data<Pool<Postgres>>,
) -> impl Responder {
  let email_id = path.into_inner();
  let confirmation_token = Uuid::new_v4();
  let email =
    match set_user_email_confirmation_token(pool.as_ref(), user.id, email_id, &confirmation_token)
      .await
    {
      Ok(Some(e)) => e,
      Ok(None) => {
        return HttpResponse::BadRequest().json(Errors::new().error("email_id", "invalid_email"));
      }
      Err(e) => {
        log::error!("{}", e);
        return HttpResponse::InternalServerError().json(Errors::internal_error());
      }
    };
  send_support_mail(
    pool.as_ref().clone(),
    application.id,
    user.username.to_owned(),
    email.email.to_owned(),
    "Confirmation Token".to_owned(),
    format!(
      r#"<h1>Welcome!</h1>
    <p>Your confirmation token is: <code>{}</code></p>"#,
      confirmation_token
    ),
  );
  HttpResponse::NoContent().finish()
}

#[utoipa::path(
  context_path = "/users",
  request_body = ResetUserPasswordRequest,
  responses(
      (status = 200, description = "Resets User's password", content_type = "text/plain", body = String),
      (status = 400, body = Errors),
      (status = 500, body = Errors),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[put("/reset-password")]
pub async fn reset_password(
  pool: Data<Pool<Postgres>>,
  user: UserRow,
  body: Json<ResetUserPasswordRequest>,
) -> impl Responder {
  if body.password != body.password_confirmation {
    return HttpResponse::BadRequest().json(Errors::from("password_confirmation_mismatch"));
  }
  let encrypted_password_result = match encrypt_password(&body.password) {
    Ok(r) => r,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  match reset_user_password(pool.as_ref(), user.id, &encrypted_password_result).await {
    Ok(_) => {}
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  HttpResponse::NoContent().finish()
}

#[utoipa::path(
  context_path = "/users",
  request_body = ChangeUsernameRequest,
  responses(
      (status = 200, description = "Changed User's username"),
      (status = 400, body = Errors),
      (status = 500, body = Errors),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[put("/change-username")]
pub async fn change_username(
  pool: Data<Pool<Postgres>>,
  user: UserRow,
  body: Json<ChangeUsernameRequest>,
) -> impl Responder {
  match change_user_username(pool.as_ref(), user.id, &body.username).await {
    Ok(_) => {}
    Err(e) => {
      log::error!("{}", e);
      match &e {
        sqlx::Error::Database(e) => {
          if let Some(_) = e.constraint() {
            return HttpResponse::BadRequest().json(Errors::from("taken"));
          }
        }
        _ => (),
      }
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  HttpResponse::NoContent().finish()
}

#[utoipa::path(
  context_path = "/users",
  params(PaginationApplicationQuery),
  responses(
      (status = 200, description = "Get current user's application", body = Vec<Application>),
      (status = 500, body = Errors),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[get("/applications")]
pub async fn applications(
  pool: Data<Pool<Postgres>>,
  user: UserRow,
  query: Query<PaginationApplicationQuery>,
) -> impl Responder {
  let applications = match get_user_applications(
    pool.as_ref(),
    user.id,
    query.page.unwrap_or(0),
    query.page_size.unwrap_or(20),
  )
  .await
  {
    Ok(e) => e,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  let applications_response: Vec<Application> = applications.into_iter().map(Into::into).collect();
  HttpResponse::Ok().json(applications_response)
}

#[utoipa::path(
  context_path = "/users",
  params(PaginationUserQuery),
  responses(
      (status = 200, description = "Get all users", body = PaginationUser),
      (status = 500, body = Errors),
  ),
  security(
      ("Authorization" = [])
  )
)]
#[get("")]
pub async fn users(
  pool: Data<Pool<Postgres>>,
  query: Query<PaginationUserQuery>,
) -> impl Responder {
  let page_size = query.page_size.unwrap_or(20);
  let users = match get_users(pool.as_ref(), query.page.unwrap_or(0), page_size).await {
    Ok(u) => u,
    Err(e) => {
      log::error!("{}", e);
      return HttpResponse::InternalServerError().json(Errors::internal_error());
    }
  };
  let users_response: Vec<User> = users
    .into_iter()
    .map(|(u, e)| (u, e, Vec::default()).into())
    .collect::<Vec<_>>();
  HttpResponse::Ok().json(PaginationUser {
    has_more: users_response.len() == page_size as usize,
    data: users_response,
  })
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
  |config: &mut ServiceConfig| {
    config.service(
      scope("/users")
        .wrap(Authorization)
        .service(current)
        .service(
          scope("")
            .wrap(AdminAppAuthorization)
            .service(create_email)
            .service(delete_email)
            .service(set_primary_email)
            .service(send_confirmation_email)
            .service(reset_password)
            .service(change_username)
            .service(applications)
            .service(scope("").wrap(AdminAuthorization).service(users)),
        ),
    );
  }
}
