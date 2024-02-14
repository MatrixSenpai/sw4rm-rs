use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Security Scheme Object
///
/// Allows the definition of a security scheme that can be used by the operations. Supported
/// schemes are basic authentication, an API key (either as a header or as a query parameter)
/// and OAuth2's common flows (implicit, password, application and access code).
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct SecurityScheme {
    /// Required. The type of the security scheme. Valid values are "basic", "apiKey" or "oauth2".
    #[serde(rename = "type")]
    pub scheme_type: String,
    /// A short description for security scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Required. The name of the header or query parameter to be used.
    pub name: String,
    /// Required The location of the API key. Valid values are "query" or "header".
    #[serde(rename = "in")]
    pub format: String,
    /// Required. The flow used by the OAuth2 security scheme. Valid values are "implicit",
    /// "password", "application" or "accessCode".
    pub flow: String,
    /// Required. The authorization URL to be used for this flow. This SHOULD be in the form
    /// of a URL.
    pub authorization_url: String,
    /// Required. The token URL to be used for this flow. This SHOULD be in the form of a URL.
    pub token_url: String,
    /// Required. The available scopes for the OAuth2 security scheme.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub scopes: HashMap<String, String>,
}