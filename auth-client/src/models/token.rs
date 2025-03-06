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
pub struct Token {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    #[serde(rename = "id_token", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id_token: Option<Option<String>>,
    #[serde(rename = "issued_at")]
    pub issued_at: String,
    #[serde(rename = "issued_token_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub issued_token_type: Option<Option<String>>,
    #[serde(rename = "refresh_token", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<Option<String>>,
    #[serde(rename = "refresh_token_expires_in", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub refresh_token_expires_in: Option<Option<i64>>,
    #[serde(rename = "scope", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Option<String>>,
    #[serde(rename = "token_type")]
    pub token_type: String,
}

impl Token {
    pub fn new(access_token: String, expires_in: i64, issued_at: String, token_type: String) -> Token {
        Token {
            access_token,
            expires_in,
            id_token: None,
            issued_at,
            issued_token_type: None,
            refresh_token: None,
            refresh_token_expires_in: None,
            scope: None,
            token_type,
        }
    }
}

