use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    Spec,
    reference::*,
    openapi_v3_0::OAuthFlows,
};

/// Security Scheme Object
///
/// Defines a security scheme that can be used by the operations. Supported schemes are HTTP
/// authentication, an API key (either as a header, a cookie parameter or as a query parameter),
/// OAuth2's common flows (implicit, password, client credentials and authorization code) as
/// defined in RFC6749, and OpenID Connect Discovery.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct SecurityScheme {
    // MARK: Common Fields
    #[serde(rename = "type")]
    pub scheme_type: SecuritySchemeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub name: String,
    #[serde(rename = "in")]
    pub location: String,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,

    // MARK: OpenAPI v2 Fields

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub scopes: HashMap<String, String>,

    // MARK: OpenAPI v3 Fields

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flows: Option<OAuthFlows>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_url: Option<String>,
}
impl Resolvable for SecurityScheme {
    fn resolve(spec: &Spec, path: &String) -> Result<Self, ResolveError> {
        let path = path.clone();
        let reference: Reference = path.clone().try_into().unwrap();

        match reference.kind {
            ReferenceType::SecurityScheme => {
                if spec.semver_check("=2") {
                    spec.security_definitions.get(&reference.name)
                        .ok_or_else(|| ResolveError::UnknownPathError(path))
                        .and_then(|t| t.resolve(spec))
                } else {
                    spec.components
                        .as_ref()
                        .ok_or_else(|| ResolveError::UnknownPathError(path.clone()))
                        .and_then(|c| c.security_schemes.get(&reference.name).ok_or_else(|| ResolveError::UnknownPathError(path)))
                        .and_then(|p| p.resolve(spec))
                }
            },
            _ => Err(ResolveError::UnknownPathError(path)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SecuritySchemeType {
    ApiKey,
    Http,
    #[serde(rename = "oauth2")]
    OAuth2,
    OpenIdConnect,
}
impl Default for SecuritySchemeType {
    fn default() -> Self { Self::ApiKey }
}
