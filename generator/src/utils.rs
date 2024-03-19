use std::{collections::HashMap, sync::Arc};

use sw4rm_rs::{shared::Schema, RefOr, Spec};


pub struct GenerationOptions;
struct Representation;

pub fn transform_files(files: Vec<super::file::File>, options: Option<GenerationOptions>) -> HashMap<String, syn::File> {
    files.into_iter()
        .flat_map(map_file_to_syn)
        .collect()
}

pub fn transform_spec(spec: Spec, options: Option<GenerationOptions>) -> Vec<super::file::File> {
    let shared_spec = Arc::new(spec);

    let spec_title = shared_spec.info.title.clone();
    info!("Parsing schema {spec_title}");

    let schema_items = shared_spec.schemas();
    debug!("{} items to parse...", schema_items.len());
    schema_items.iter()
        .flat_map(|(key, schema)| resolve(shared_spec.clone(), key, schema))
        .flat_map(|(key, schema)| parse_definitions(shared_spec.clone(), key, schema))
        .inspect(|definition| info!("{definition:?}"))
        .collect::<Vec<_>>()
}

fn map_file_to_syn(file: super::file::File) -> Option<(String, syn::File)> {
    let name = file.file_name.clone().change_rep(convert_case::Case::UpperCamel, false).0;
    debug!("Converting {} to syn file representation", &name);

    let f = match file.try_into() {
        Ok(v) => v,
        Err(e) => {
            error!("Could not transform {} into syn file: {e:?}", &name);
            return None
        }
    };

    Some((name, f))
}

pub(crate) fn resolve(spec: Arc<Spec>, key: &String, reference: &RefOr<Schema>) -> Option<(String, Schema)> {
    trace!("RESOLUTION: {key}");

    let reference = match reference.resolve(&spec) {
        Ok(r) => r,
        Err(e) => {
            error!("Attempted to resolve {key} failed: {e:?}");
            return None;
        }
    };

    Some((key.clone(), reference))
}

pub(crate) fn resolve_boxed(spec: Arc<Spec>, key: &String, reference: &RefOr<Box<Schema>>) -> Option<(String, Box<Schema>)> {
    trace!("RESOLUTION: {key}");

    let reference = match reference.resolve(&spec) {
        Ok(r) => r,
        Err(e) => {
            error!("Attempted to resolve {key} failed: {e:?}");
            return None;
        }
    };

    Some((key.clone(), reference))
}

fn parse_definitions(spec: Arc<Spec>, definition_key: String, schema_item: Schema) -> Option<super::file::File> {
    trace!("Parsing model for {}", &definition_key);

    super::file::FileInputParams {
        definition_key, schema_item, spec: spec.clone()
    }.try_into().ok()
}
