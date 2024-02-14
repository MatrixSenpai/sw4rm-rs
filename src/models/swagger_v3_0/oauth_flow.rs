use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OAuth Flows Object
///
/// Allows configuration of the supported OAuth Flows.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct OAuthFlows {
    /// Configuration for the OAuth Implicit flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<OAuthFlow>,
    /// Configuration for the OAuth Resource Owner Password flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<OAuthFlow>,
    /// Configuration for the OAuth Client Credentials flow. Previously called application in
    /// OpenAPI 2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<OAuthFlow>,
    /// Configuration for the OAuth Authorization Code flow. Previously called accessCode in
    /// OpenAPI 2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<OAuthFlow>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}

/// OAuth Flow Object
///
/// Configuration details for a supported OAuth Flow
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct OAuthFlow {
    /// REQUIRED. The authorization URL to be used for this flow. This MUST be in the form of a URL.
    pub authorization_url: String,
    /// REQUIRED. The token URL to be used for this flow. This MUST be in the form of a URL.
    pub token_url: String,
    /// The URL to be used for obtaining refresh tokens. This MUST be in the form of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    /// REQUIRED. The available scopes for the OAuth2 security scheme. A map between the scope
    /// name and a short description for it. The map MAY be empty.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub scopes: HashMap<String, String>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}