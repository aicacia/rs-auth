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


/// struct for typed errors of method [`all_applications`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllApplicationsError {
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateApplicationError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteApplicationError {
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status404(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_application_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetApplicationByIdError {
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status404(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateApplicationError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}


pub async fn all_applications(configuration: &configuration::Configuration, offset: Option<u32>, limit: Option<u32>) -> Result<models::ApplicationPagination, Error<AllApplicationsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_offset = offset;
    let p_limit = limit;

    let uri_str = format!("{}/applications", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApplicationPagination`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApplicationPagination`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AllApplicationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_application(configuration: &configuration::Configuration, create_application: models::CreateApplication) -> Result<models::Application, Error<CreateApplicationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_create_application = create_application;

    let uri_str = format!("{}/applications", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_create_application);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Application`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Application`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateApplicationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_application(configuration: &configuration::Configuration, application_id: i64) -> Result<(), Error<DeleteApplicationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_application_id = application_id;

    let uri_str = format!("{}/applications/{application_id}", configuration.base_path, application_id=p_application_id);
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
        let entity: Option<DeleteApplicationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn get_application_by_id(configuration: &configuration::Configuration, application_id: i64) -> Result<models::Application, Error<GetApplicationByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_application_id = application_id;

    let uri_str = format!("{}/applications/{application_id}", configuration.base_path, application_id=p_application_id);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Application`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Application`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetApplicationByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_application(configuration: &configuration::Configuration, application_id: i64, update_application: models::UpdateApplication) -> Result<models::Application, Error<UpdateApplicationError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_application_id = application_id;
    let p_update_application = update_application;

    let uri_str = format!("{}/applications/{application_id}", configuration.base_path, application_id=p_application_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_update_application);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Application`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Application`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateApplicationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

