/*
 * auth
 *
 * Aicacia Auth API provides authentication services for applications.
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTenantOAuth2Provider {
    #[serde(rename = "active", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub active: Option<Option<bool>>,
    #[serde(rename = "auth_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub auth_url: Option<Option<String>>,
    #[serde(rename = "callback_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<Option<String>>,
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "client_secret")]
    pub client_secret: String,
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "redirect_url")]
    pub redirect_url: String,
    #[serde(rename = "scope", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Option<String>>,
    #[serde(rename = "token_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub token_url: Option<Option<String>>,
}

impl CreateTenantOAuth2Provider {
    pub fn new(client_id: String, client_secret: String, provider: String, redirect_url: String) -> CreateTenantOAuth2Provider {
        CreateTenantOAuth2Provider {
            active: None,
            auth_url: None,
            callback_url: None,
            client_id,
            client_secret,
            provider,
            redirect_url,
            scope: None,
            token_url: None,
        }
    }
}

