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
pub struct UserInfo {
    #[serde(rename = "address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address: Option<Option<String>>,
    #[serde(rename = "birthdate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<Option<String>>,
    #[serde(rename = "family_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub family_name: Option<Option<String>>,
    #[serde(rename = "gender", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub gender: Option<Option<String>>,
    #[serde(rename = "given_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub given_name: Option<Option<String>>,
    #[serde(rename = "locale", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub locale: Option<Option<String>>,
    #[serde(rename = "middle_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<Option<String>>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "nickname", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<Option<String>>,
    #[serde(rename = "profile_picture", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<Option<String>>,
    #[serde(rename = "website", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub website: Option<Option<String>>,
    #[serde(rename = "zone_info", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub zone_info: Option<Option<String>>,
}

impl UserInfo {
    pub fn new() -> UserInfo {
        UserInfo {
            address: None,
            birthdate: None,
            family_name: None,
            gender: None,
            given_name: None,
            locale: None,
            middle_name: None,
            name: None,
            nickname: None,
            profile_picture: None,
            website: None,
            zone_info: None,
        }
    }
}

