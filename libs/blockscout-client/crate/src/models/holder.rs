/*
 * BlockScout API
 *
 * API for BlockScout web app
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: you@your-company.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Holder {
    #[serde(rename = "address")]
    pub address: models::AddressParam,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "token_id", skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(rename = "token")]
    pub token: models::TokenInfo,
}

impl Holder {
    pub fn new(address: models::AddressParam, value: String, token: models::TokenInfo) -> Holder {
        Holder {
            address,
            value,
            token_id: None,
            token,
        }
    }
}
