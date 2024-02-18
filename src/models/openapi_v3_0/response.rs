use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use resolve_core::RefOr;
use super::{
    parameter::Parameter,
    media_type::MediaType,
    link::Link,
};

/// Response Object
///
/// Describes a single response from an API Operation, including design-time, static links to
/// operations based on the response.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Resolvable)]
#[resolve(reference_type = "response")]
#[serde(default, rename_all = "camelCase")]
pub struct Response {
    /// REQUIRED. A short description of the response. CommonMark syntax MAY be used for rich
    /// text representation.
    pub description: String,
    /// Maps a header name to its definition. RFC7230 states header names are case-insensitive.
    /// If a response header is defined with the name "Content-Type", it SHALL be ignored.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, RefOr<Parameter>>,
    /// A map containing descriptions of potential response payloads. The key is a media type or
    /// media type range and the value describes it. For responses that match multiple keys, only
    /// the most specific key is applicable. e.g. text/plain overrides text/*
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub content: HashMap<String, MediaType>,
    /// A map of operations links that can be followed from the response. The key of the map is a
    /// short name for the link, following the naming constraints of the names for Component
    /// Objects.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub links: HashMap<String, RefOr<Link>>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}
