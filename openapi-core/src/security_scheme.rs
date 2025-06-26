use crate::parameter::ParameterLocation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct SecurityScheme {
    #[serde(rename = "type")]
    pub scheme_type: SecuritySchemeType,
    pub description: Option<String>,
    pub name: Option<String>,
    pub location: Option<ParameterLocation>,
    pub scheme: Option<String>,
    pub bearer_format: Option<String>,
    pub flows: HashMap<OAuthFlowKey, OAuthFlow>,
    pub open_id_connect_url: Option<String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SecuritySchemeType {
    #[default]
    ApiKey,
    Http,
    MutualTLS,
    Oauth2,
    OpenIdConnect,
    Unknown(String),
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OAuthFlowKey {
    Implicit,
    Password,
    ClientCredentials,
    AuthorizationCode,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct OAuthFlow {
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
