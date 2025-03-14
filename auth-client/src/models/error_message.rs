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
pub struct ErrorMessage {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "parameters")]
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

impl ErrorMessage {
    pub fn new(code: String, parameters: std::collections::HashMap<String, serde_json::Value>) -> ErrorMessage {
        ErrorMessage {
            code,
            parameters,
        }
    }
}

