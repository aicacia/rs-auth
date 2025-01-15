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
pub struct CreateTotpRequest {
    #[serde(rename = "algorithm", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<Option<String>>,
    #[serde(rename = "digits", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub digits: Option<Option<i64>>,
    #[serde(rename = "step", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub step: Option<Option<i64>>,
}

impl CreateTotpRequest {
    pub fn new() -> CreateTotpRequest {
        CreateTotpRequest {
            algorithm: None,
            digits: None,
            step: None,
        }
    }
}

