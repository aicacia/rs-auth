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
pub struct UserEmail {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "primary")]
    pub primary: bool,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "verified")]
    pub verified: bool,
}

impl UserEmail {
    pub fn new(created_at: String, email: String, id: i64, primary: bool, updated_at: String, verified: bool) -> UserEmail {
        UserEmail {
            created_at,
            email,
            id,
            primary,
            updated_at,
            verified,
        }
    }
}

