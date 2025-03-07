/*
 * auth
 *
 * Aicacia Auth API provides authentication services for applications.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`create_current_user_add_oauth2_provider_url`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCurrentUserAddOauth2ProviderUrlError {
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_current_user_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCurrentUserEmailError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status409(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_current_user_phone_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCurrentUserPhoneNumberError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status409(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_current_user_totp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCurrentUserTotpError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status409(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`deactivate_current_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeactivateCurrentUserError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_current_user_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCurrentUserEmailError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status409(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_current_user_phone_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCurrentUserPhoneNumberError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status409(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_current_user_totp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCurrentUserTotpError {
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status404(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_current_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCurrentUserError {
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`reset_current_user_password`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResetCurrentUserPasswordError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_current_user_email_as_primary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetCurrentUserEmailAsPrimaryError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status409(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_current_user_phone_number_as_primary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetCurrentUserPhoneNumberAsPrimaryError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status409(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_current_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCurrentUserError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_current_user_config`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCurrentUserConfigError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status409(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_current_user_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCurrentUserInfoError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}


pub async fn create_current_user_add_oauth2_provider_url(configuration: &configuration::Configuration, provider: &str, state: Option<&str>) -> Result<String, Error<CreateCurrentUserAddOauth2ProviderUrlError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_provider = provider;
    let p_state = state;

    let uri_str = format!("{}/current-user/oauth2/{provider}", configuration.base_path, provider=crate::apis::urlencode(p_provider));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_state {
        req_builder = req_builder.query(&[("state", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Ok(content),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `String`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateCurrentUserAddOauth2ProviderUrlError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_current_user_email(configuration: &configuration::Configuration, create_user_email: models::CreateUserEmail) -> Result<models::UserEmail, Error<CreateCurrentUserEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_create_user_email = create_user_email;

    let uri_str = format!("{}/current-user/emails", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_create_user_email);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UserEmail`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UserEmail`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateCurrentUserEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_current_user_phone_number(configuration: &configuration::Configuration, create_user_phone_number: models::CreateUserPhoneNumber) -> Result<models::UserPhoneNumber, Error<CreateCurrentUserPhoneNumberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_create_user_phone_number = create_user_phone_number;

    let uri_str = format!("{}/current-user/phone-numbers", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_create_user_phone_number);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UserPhoneNumber`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UserPhoneNumber`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateCurrentUserPhoneNumberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_current_user_totp(configuration: &configuration::Configuration, create_totp_request: models::CreateTotpRequest) -> Result<models::UserTotp, Error<CreateCurrentUserTotpError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_create_totp_request = create_totp_request;

    let uri_str = format!("{}/current-user/totp", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_create_totp_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UserTotp`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UserTotp`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateCurrentUserTotpError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn deactivate_current_user(configuration: &configuration::Configuration, ) -> Result<(), Error<DeactivateCurrentUserError>> {

    let uri_str = format!("{}/current-user", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeactivateCurrentUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_current_user_email(configuration: &configuration::Configuration, email_id: i64) -> Result<(), Error<DeleteCurrentUserEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_email_id = email_id;

    let uri_str = format!("{}/current-user/emails/{email_id}", configuration.base_path, email_id=p_email_id);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteCurrentUserEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_current_user_phone_number(configuration: &configuration::Configuration, phone_number_id: i64) -> Result<(), Error<DeleteCurrentUserPhoneNumberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_phone_number_id = phone_number_id;

    let uri_str = format!("{}/current-user/phone-numbers/{phone_number_id}", configuration.base_path, phone_number_id=p_phone_number_id);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteCurrentUserPhoneNumberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_current_user_totp(configuration: &configuration::Configuration, ) -> Result<(), Error<DeleteCurrentUserTotpError>> {

    let uri_str = format!("{}/current-user/totp", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteCurrentUserTotpError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_current_user(configuration: &configuration::Configuration, ) -> Result<models::User, Error<GetCurrentUserError>> {

    let uri_str = format!("{}/current-user", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::User`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::User`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCurrentUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn reset_current_user_password(configuration: &configuration::Configuration, reset_password_request: models::ResetPasswordRequest) -> Result<(), Error<ResetCurrentUserPasswordError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_reset_password_request = reset_password_request;

    let uri_str = format!("{}/current-user/reset-password", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_reset_password_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<ResetCurrentUserPasswordError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn set_current_user_email_as_primary(configuration: &configuration::Configuration, email_id: i64) -> Result<(), Error<SetCurrentUserEmailAsPrimaryError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_email_id = email_id;

    let uri_str = format!("{}/current-user/emails/{email_id}/set-as-primary", configuration.base_path, email_id=p_email_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<SetCurrentUserEmailAsPrimaryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn set_current_user_phone_number_as_primary(configuration: &configuration::Configuration, phone_number_id: i64) -> Result<(), Error<SetCurrentUserPhoneNumberAsPrimaryError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_phone_number_id = phone_number_id;

    let uri_str = format!("{}/current-user/phone-numbers/{phone_number_id}/set-as-primary", configuration.base_path, phone_number_id=p_phone_number_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<SetCurrentUserPhoneNumberAsPrimaryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_current_user(configuration: &configuration::Configuration, update_user: models::UpdateUser) -> Result<(), Error<UpdateCurrentUserError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_update_user = update_user;

    let uri_str = format!("{}/current-user", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_update_user);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateCurrentUserError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_current_user_config(configuration: &configuration::Configuration, update_user_config_request: models::UpdateUserConfigRequest) -> Result<(), Error<UpdateCurrentUserConfigError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_update_user_config_request = update_user_config_request;

    let uri_str = format!("{}/current-user/config", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_update_user_config_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateCurrentUserConfigError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_current_user_info(configuration: &configuration::Configuration, update_user_info_request: models::UpdateUserInfoRequest) -> Result<(), Error<UpdateCurrentUserInfoError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_update_user_info_request = update_user_info_request;

    let uri_str = format!("{}/current-user/info", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_update_user_info_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateCurrentUserInfoError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

