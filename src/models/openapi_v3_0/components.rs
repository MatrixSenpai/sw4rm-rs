use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::{
    RefOr,
    shared::{
        Callback,
        Parameter,
        Response,
        Schema,
        SecurityScheme,
    },
    openapi_v3_0::{
        Example,
        Link,
        RequestBody,
    }
};

/// Components Object
///
/// Holds a set of reusable objects for different aspects of the OAS. All objects defined within
/// the components object will have no effect on the API unless they are explicitly referenced
/// from properties outside the components object.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct Components {
    /// An object to hold reusable Schema Objects.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub schemas: HashMap<String, RefOr<Schema>>,
    /// An object to hold reusable Response Objects.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub responses: HashMap<String, RefOr<Response>>,
    /// An object to hold reusable Parameter Objects.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, RefOr<Parameter>>,
    /// An object to hold reusable Example Objects.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub examples: HashMap<String, RefOr<Example>>,
    /// An object to hold reusable Request Body Objects.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub request_bodies: HashMap<String, RefOr<RequestBody>>,
    /// An object to hold reusable Header Objects.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, RefOr<Parameter>>,
    /// An object to hold reusable Security Scheme Objects.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub security_schemes: HashMap<String, RefOr<SecurityScheme>>,
    /// An object to hold reusable Link Objects.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub links: HashMap<String, RefOr<Link>>,
    /// An object to hold reusable Callback Objects.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub callbacks: HashMap<String, RefOr<Callback>>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}