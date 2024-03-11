mod model;
mod file;
mod import;
mod attribute;
mod field;

use sw4rm_rs::{
    *, shared::*, openapi_v2::*, openapi_v3_0::*,
};

pub struct GenerationOptions;
struct Representation;

pub fn parse_spec(spec: Spec, options: Option<GenerationOptions>) {
    let spec_title = spec.info.title.clone();
    info!("Parsing schema {spec_title}");

    // TODO: maybe use flat_map here...?
    spec.schemas().iter()
        .map(|(key, schema)| resolve(&spec, key, schema))
        .map(|(key, schema)| parse_definitions(&spec, key, schema));
}

// TODO: this should return Option instead
fn resolve(spec: &Spec, key: &String, reference: &RefOr<Schema>) -> (String, Schema) {
    debug!("RESOLUTION: {key}");
    (
        key.clone(),
        reference.resolve(spec).unwrap(),
    )
}

// TODO: this should return Option instead
fn parse_definitions(spec: &Spec, definition_key: String, schema_item: Schema) {
    match schema_item.schema_type.unwrap() {
        // SchemaType::Object => parse_schema_object(spec, definition_key, schema_item),
        _ => ()
    };
}

fn parse_schema_object(spec: &Spec, definition_key: String, schema_item: Schema) {
    info!("Parsing model for {}", &definition_key);

    let required = schema_item.required;
    let properties = schema_item.properties;
}
