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
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`create_user_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUserEmailError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status409(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_user_phone_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUserPhoneNumberError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status409(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_user_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserEmailError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_user_phone_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserPhoneNumberError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_user_email`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserEmailError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_user_phone_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserPhoneNumberError {
    Status400(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status401(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    Status500(std::collections::HashMap<String, Vec<models::ErrorMessage>>),
    UnknownValue(serde_json::Value),
}


pub async fn create_user_email(configuration: &configuration::Configuration, user_id: i64, service_account_create_user_email: models::ServiceAccountCreateUserEmail, application_id: Option<i64>) -> Result<models::UserEmail, Error<CreateUserEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_user_id = user_id;
    let p_service_account_create_user_email = service_account_create_user_email;
    let p_application_id = application_id;

    let uri_str = format!("{}/users/{user_id}/emails", configuration.base_path, user_id=p_user_id);
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
    req_builder = req_builder.json(&p_service_account_create_user_email);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateUserEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn create_user_phone_number(configuration: &configuration::Configuration, user_id: i64, service_account_create_user_phone_number: models::ServiceAccountCreateUserPhoneNumber, application_id: Option<i64>) -> Result<models::UserPhoneNumber, Error<CreateUserPhoneNumberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_user_id = user_id;
    let p_service_account_create_user_phone_number = service_account_create_user_phone_number;
    let p_application_id = application_id;

    let uri_str = format!("{}/users/{user_id}/phone_numbers", configuration.base_path, user_id=p_user_id);
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
    req_builder = req_builder.json(&p_service_account_create_user_phone_number);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateUserPhoneNumberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_user_email(configuration: &configuration::Configuration, user_id: i64, email_id: i64, application_id: Option<i64>) -> Result<(), Error<DeleteUserEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_user_id = user_id;
    let p_email_id = email_id;
    let p_application_id = application_id;

    let uri_str = format!("{}/users/{user_id}/emails/{email_id}", configuration.base_path, user_id=p_user_id, email_id=p_email_id);
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
        let entity: Option<DeleteUserEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn delete_user_phone_number(configuration: &configuration::Configuration, user_id: i64, phone_number_id: i64, application_id: Option<i64>) -> Result<(), Error<DeleteUserPhoneNumberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_user_id = user_id;
    let p_phone_number_id = phone_number_id;
    let p_application_id = application_id;

    let uri_str = format!("{}/users/{user_id}/phone-numbers/{phone_number_id}", configuration.base_path, user_id=p_user_id, phone_number_id=p_phone_number_id);
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
        let entity: Option<DeleteUserPhoneNumberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_user_email(configuration: &configuration::Configuration, user_id: i64, email_id: i64, service_account_update_user_email: models::ServiceAccountUpdateUserEmail, application_id: Option<i64>) -> Result<(), Error<UpdateUserEmailError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_user_id = user_id;
    let p_email_id = email_id;
    let p_service_account_update_user_email = service_account_update_user_email;
    let p_application_id = application_id;

    let uri_str = format!("{}/users/{user_id}/emails/{email_id}", configuration.base_path, user_id=p_user_id, email_id=p_email_id);
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
    req_builder = req_builder.json(&p_service_account_update_user_email);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateUserEmailError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn update_user_phone_number(configuration: &configuration::Configuration, user_id: i64, phone_number_id: i64, service_account_update_user_phone_number: models::ServiceAccountUpdateUserPhoneNumber, application_id: Option<i64>) -> Result<(), Error<UpdateUserPhoneNumberError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_user_id = user_id;
    let p_phone_number_id = phone_number_id;
    let p_service_account_update_user_phone_number = service_account_update_user_phone_number;
    let p_application_id = application_id;

    let uri_str = format!("{}/users/{user_id}/phone-numbers/{phone_number_id}", configuration.base_path, user_id=p_user_id, phone_number_id=p_phone_number_id);
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
    req_builder = req_builder.json(&p_service_account_update_user_phone_number);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateUserPhoneNumberError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

