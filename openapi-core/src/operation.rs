use crate::RefOr;
use crate::external_documentation::ExternalDocumentation;
use crate::media_type::Callback;
use crate::parameter::Parameter;
use crate::request_body::RequestBody;
use crate::response::Responses;
use crate::security_requirement::SecurityRequirement;
use crate::server::Server;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Operation {
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub external_docs: Option<ExternalDocumentation>,
    pub operation_id: Option<String>,
    pub parameters: Vec<RefOr<Parameter>>,
    pub request_body: Option<RefOr<RequestBody>>,
    pub responses: Responses,
    pub callbacks: HashMap<String, RefOr<Callback>>,
    pub deprecated: Option<bool>,
    pub security: Vec<SecurityRequirement>,
    pub servers: Vec<Server>,

    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
