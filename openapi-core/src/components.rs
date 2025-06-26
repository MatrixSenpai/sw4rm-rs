use crate::RefOr;
use crate::example::Example;
use crate::header::Header;
use crate::link::Link;
use crate::media_type::Callback;
use crate::parameter::Parameter;
use crate::path_item::Paths;
use crate::request_body::RequestBody;
use crate::response::Response;
use crate::schema::Schema;
use crate::security_scheme::SecurityScheme;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Components {
    pub schemas: HashMap<String, RefOr<Schema>>,
    pub responses: HashMap<String, RefOr<Response>>,
    pub parameters: HashMap<String, RefOr<Parameter>>,
    pub examples: HashMap<String, RefOr<Example>>,
    pub request_bodies: HashMap<String, RefOr<RequestBody>>,
    pub headers: HashMap<String, RefOr<Header>>,
    pub security_schemes: HashMap<String, RefOr<SecurityScheme>>,
    pub links: HashMap<String, RefOr<Link>>,
    pub callbacks: HashMap<String, RefOr<Callback>>,
    pub path_items: Paths,
    
    #[serde(flatten)]
    pub additional_properties: HashMap<String, Value>,
}
