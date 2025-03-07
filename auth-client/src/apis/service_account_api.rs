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


/// struct for typed errors of method [`all_service_accounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllServiceAccountsError {
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_service_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateServiceAccountError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_service_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteServiceAccountError {
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status404(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_service_account_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceAccountByIdError {
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status404(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_service_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateServiceAccountError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}


pub async fn all_service_accounts(configuration: &configuration::Configuration, offset: Option<u32>, limit: Option<u32>, application_id: Option<i64>) -> Result<models::ServiceAccountPagination, Error<AllServiceAccountsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_offset = offset;
    let p_limit = limit;
    let p_application_id = application_id;

    let uri_str = format!("{}/service-accounts", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_application_id {
        req_builder = req_builder.query(&[("application_id", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ServiceAccountPagination`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ServiceAccountPagination`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AllServiceAccountsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_service_account(configuration: &configuration::Configuration, create_service_account: models::CreateServiceAccount, application_id: Option<i64>) -> Result<models::ServiceAccount, Error<CreateServiceAccountError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_create_service_account = create_service_account;
    let p_application_id = application_id;

    let uri_str = format!("{}/service-accounts", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref param_value) = p_application_id {
        req_builder = req_builder.query(&[("application_id", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_create_service_account);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ServiceAccount`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ServiceAccount`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateServiceAccountError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_service_account(configuration: &configuration::Configuration, service_account_id: i64, application_id: Option<i64>) -> Result<(), Error<DeleteServiceAccountError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_service_account_id = service_account_id;
    let p_application_id = application_id;

    let uri_str = format!("{}/service-accounts/{service_account_id}", configuration.base_path, service_account_id=p_service_account_id);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref param_value) = p_application_id {
        req_builder = req_builder.query(&[("application_id", &param_value.to_string())]);
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

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteServiceAccountError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_service_account_by_id(configuration: &configuration::Configuration, service_account_id: i64, application_id: Option<i64>) -> Result<models::ServiceAccount, Error<GetServiceAccountByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_service_account_id = service_account_id;
    let p_application_id = application_id;

    let uri_str = format!("{}/service-accounts/{service_account_id}", configuration.base_path, service_account_id=p_service_account_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_application_id {
        req_builder = req_builder.query(&[("application_id", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ServiceAccount`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ServiceAccount`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetServiceAccountByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_service_account(configuration: &configuration::Configuration, service_account_id: i64, update_service_account: models::UpdateServiceAccount, application_id: Option<i64>) -> Result<models::ServiceAccount, Error<UpdateServiceAccountError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_service_account_id = service_account_id;
    let p_update_service_account = update_service_account;
    let p_application_id = application_id;

    let uri_str = format!("{}/service-accounts/{service_account_id}", configuration.base_path, service_account_id=p_service_account_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref param_value) = p_application_id {
        req_builder = req_builder.query(&[("application_id", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_update_service_account);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ServiceAccount`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ServiceAccount`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateServiceAccountError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

