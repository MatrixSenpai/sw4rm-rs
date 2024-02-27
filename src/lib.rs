pub mod models;
pub mod error;
pub mod generation;

use std::{
    fs::File, io::Read, path::Path,
};

use crate::{
    models::Spec,
    error::Error
};

pub fn from_path<P>(path: P) -> Result<Spec, Error> where P: AsRef<Path> {
    from_reader(File::open(path)?)
}
pub fn from_reader<R>(reader: R) -> Result<Spec, Error> where R: Read {
    Ok(serde_yaml::from_reader(reader)?)
}

#[cfg(test)]
mod tests {
    use crate::models::{
        reference::*,
        shared::{
            Schema,
            Parameter,
        },
    };

    #[test]
    fn deserialize_swagger_v2_yaml() {
        let spec = super::from_path("./resources/swagger_2_0.yaml").unwrap();
        let reference: RefOr<Parameter> = RefOr::Reference { reference_path: "#/parameters/nested_response".to_string() };
        _ = reference.resolve(&spec).unwrap();
    }
    #[test]
    fn deserialize_swagger_v2_json() {
        let spec = super::from_path("./resources/riot-swaggerspec-2.0.json").unwrap();
        let reference: RefOr<Schema> = RefOr::Reference { reference_path: "#/definitions/account-v1.AccountDto".to_string() };
        _ = reference.resolve(&spec).unwrap();
    }
    #[test]
    fn deserialize_openapi_v3_yaml() {
        let spec = super::from_path("./resources/openapi_3_0.yaml").unwrap();
        let reference: RefOr<Schema> = RefOr::Reference { reference_path: "#/components/schemas/Pets".to_string() };
        _ = reference.resolve(&spec).unwrap();
    }
    #[test]
    fn deserialize_openapi_v3_json() {
        let spec = super::from_path("./resources/riot-openapi-3.0.0.json").unwrap();
        let reference: RefOr<Schema> = RefOr::Reference { reference_path: "#/components/schemas/account-v1.AccountDto".to_string() };
        _ = reference.resolve(&spec).unwrap();
    }
}