use tokio::fs;

use crate::{
  core::{
    config::Config,
    encryption::encrypt_password,
    error::{Errors, DATEBASE_ERROR, INTERNAL_ERROR},
  },
  model::service_account::ServiceAccount,
  repository::service_account::{
    create_service_account, get_service_accounts, CreateServiceAccount,
  },
};

pub async fn init_service_accounts(pool: &sqlx::AnyPool, config: &Config) -> Result<(), Errors> {
  let service_accounts = match get_service_accounts(pool, None, None).await {
    Ok(service_accounts) => service_accounts,
    Err(e) => {
      log::error!("Error getting service accounts: {}", e);
      return Err(Errors::internal_error().with_application_error(DATEBASE_ERROR));
    }
  };
  if !service_accounts.is_empty() {
    return Ok(());
  }
  log::info!("No service accounts found, creating initial admin service account");
  create_new_admin_service_account(pool, config).await
}

pub async fn create_new_admin_service_account(
  pool: &sqlx::AnyPool,
  config: &Config,
) -> Result<(), Errors> {
  let client_id = uuid::Uuid::new_v4();
  let client_secret = uuid::Uuid::new_v4();
  let encrypted_client_secret = match encrypt_password(config, &client_secret.to_string()) {
    Ok(encrypted_client_secret) => encrypted_client_secret,
    Err(e) => {
      log::error!("Error encrypting client secret: {}", e);
      return Err(Errors::internal_error().with_application_error(DATEBASE_ERROR));
    }
  };
  let service_account_row = match create_service_account(
    pool,
    CreateServiceAccount {
      client_id: client_id.to_string(),
      encrypted_client_secret,
      name: "Admin".to_owned(),
    },
  )
  .await
  {
    Ok(row) => row,
    Err(e) => {
      log::error!("Error creating service account: {}", e);
      return Err(Errors::internal_error().with_application_error(DATEBASE_ERROR));
    }
  };
  let mut service_account = ServiceAccount::from(service_account_row);
  service_account.client_secret = Some(client_secret.clone());
  let service_account_json_string = match serde_json::to_string_pretty(&service_account) {
    Ok(json) => json,
    Err(e) => {
      log::error!("Error serializing service account: {}", e);
      return Err(Errors::internal_error().with_application_error(INTERNAL_ERROR));
    }
  };
  match fs::write(
    "./auth-admin-service-account.json",
    service_account_json_string,
  )
  .await
  {
    Ok(_) => {}
    Err(e) => {
      log::error!("Error writing service account to file: {}", e);
      return Err(Errors::internal_error().with_application_error(INTERNAL_ERROR));
    }
  }
  log::info!("Service account created, see auth-admin-service-account.json");
  Ok(())
}
