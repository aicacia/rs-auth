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


/// struct for typed errors of method [`all_tenants`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllTenantsError {
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_tenant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTenantError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_tenant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTenantError {
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status404(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tenant_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTenantByIdError {
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status404(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_tenant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTenantError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}


pub async fn all_tenants(configuration: &configuration::Configuration, offset: Option<u32>, limit: Option<u32>, show_private_key: Option<bool>, application_id: Option<i64>) -> Result<models::TenantPagination, Error<AllTenantsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_offset = offset;
    let p_limit = limit;
    let p_show_private_key = show_private_key;
    let p_application_id = application_id;

    let uri_str = format!("{}/tenants", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_show_private_key {
        req_builder = req_builder.query(&[("show_private_key", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::TenantPagination`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::TenantPagination`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AllTenantsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_tenant(configuration: &configuration::Configuration, create_tenant: models::CreateTenant, application_id: Option<i64>) -> Result<models::Tenant, Error<CreateTenantError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_create_tenant = create_tenant;
    let p_application_id = application_id;

    let uri_str = format!("{}/tenants", configuration.base_path);
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
    req_builder = req_builder.json(&p_create_tenant);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Tenant`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Tenant`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateTenantError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_tenant(configuration: &configuration::Configuration, tenant_id: i64, application_id: Option<i64>) -> Result<(), Error<DeleteTenantError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_tenant_id = tenant_id;
    let p_application_id = application_id;

    let uri_str = format!("{}/tenants/{tenant_id}", configuration.base_path, tenant_id=p_tenant_id);
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
        let entity: Option<DeleteTenantError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_tenant_by_id(configuration: &configuration::Configuration, tenant_id: i64, show_private_key: Option<bool>, application_id: Option<i64>) -> Result<models::Tenant, Error<GetTenantByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_tenant_id = tenant_id;
    let p_show_private_key = show_private_key;
    let p_application_id = application_id;

    let uri_str = format!("{}/tenants/{tenant_id}", configuration.base_path, tenant_id=p_tenant_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_show_private_key {
        req_builder = req_builder.query(&[("show_private_key", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Tenant`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Tenant`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTenantByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_tenant(configuration: &configuration::Configuration, tenant_id: i64, update_tenant: models::UpdateTenant, application_id: Option<i64>) -> Result<models::Tenant, Error<UpdateTenantError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_tenant_id = tenant_id;
    let p_update_tenant = update_tenant;
    let p_application_id = application_id;

    let uri_str = format!("{}/tenants/{tenant_id}", configuration.base_path, tenant_id=p_tenant_id);
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
    req_builder = req_builder.json(&p_update_tenant);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Tenant`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Tenant`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateTenantError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

