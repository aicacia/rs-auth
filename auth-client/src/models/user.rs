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
pub struct User {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "config", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub config: Option<Option<models::UserConfig>>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "email", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<models::UserEmail>>,
    #[serde(rename = "emails")]
    pub emails: Vec<models::UserEmail>,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "info")]
    pub info: models::UserInfo,
    #[serde(rename = "mfa_types")]
    pub mfa_types: Vec<models::UserMfaType>,
    #[serde(rename = "oauth2_providers")]
    pub oauth2_providers: Vec<models::UserOAuth2Provider>,
    #[serde(rename = "phone_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Option<models::UserPhoneNumber>>,
    #[serde(rename = "phone_numbers")]
    pub phone_numbers: Vec<models::UserPhoneNumber>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "username")]
    pub username: String,
}

impl User {
    pub fn new(active: bool, created_at: String, emails: Vec<models::UserEmail>, id: i64, info: models::UserInfo, mfa_types: Vec<models::UserMfaType>, oauth2_providers: Vec<models::UserOAuth2Provider>, phone_numbers: Vec<models::UserPhoneNumber>, updated_at: String, username: String) -> User {
        User {
            active,
            config: None,
            created_at,
            email: None,
            emails,
            id,
            info,
            mfa_types,
            oauth2_providers,
            phone_number: None,
            phone_numbers,
            updated_at,
            username,
        }
    }
}

