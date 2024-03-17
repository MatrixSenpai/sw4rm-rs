use sw4rm_rs::{
    from_path,
    reference::*,
    shared::{
        Schema,
        Parameter,
    },
};

#[test]
fn deserialize_swagger_v2_yaml() {
    let spec = from_path("../resources/swagger_2_0.yaml").unwrap();
    let reference: RefOr<Parameter> = RefOr::Reference { reference_path: "#/parameters/nested_response".to_string() };
    _ = reference.resolve(&spec).unwrap();

    assert_eq!(
        &spec.schemas().len(),
        &spec.definitions.len(),
    );
    assert_ne!(
        &spec.schemas().len(),
        &0
    );
}
#[test]
fn deserialize_swagger_v2_json() {
    let spec = from_path("../resources/riot-swaggerspec-2.0.json").unwrap();
    let reference: RefOr<Schema> = RefOr::Reference { reference_path: "#/definitions/account-v1.AccountDto".to_string() };
    _ = reference.resolve(&spec).unwrap();

    assert_eq!(
        &spec.schemas().len(),
        &spec.definitions.len(),
    );
    assert_ne!(
        &spec.schemas().len(),
        &0
    );
}
#[test]
fn deserialize_openapi_v3_yaml() {
    let spec = from_path("../resources/openapi_3_0.yaml").unwrap();
    let reference: RefOr<Schema> = RefOr::Reference { reference_path: "#/components/schemas/Pets".to_string() };
    _ = reference.resolve(&spec).unwrap();

    assert_eq!(
        &spec.schemas().len(),
        &spec.components.clone().map(|c| c.schemas.len()).unwrap_or(0),
    );
    assert_ne!(
        &spec.schemas().len(),
        &0
    );
}
#[test]
fn deserialize_openapi_v3_json() {
    let spec = from_path("../resources/riot-openapi-3.0.0.json").unwrap();
    let reference: RefOr<Schema> = RefOr::Reference { reference_path: "#/components/schemas/account-v1.AccountDto".to_string() };
    _ = reference.resolve(&spec).unwrap();

    assert_eq!(
        &spec.schemas().len(),
        &spec.components.clone().map(|c| c.schemas.len()).unwrap_or(0),
    );
    assert_ne!(
        &spec.schemas().len(),
        &0
    );
}

#[test]
fn deserialize_discord_json() {
    let spec = from_path("../resources/discord-spec.json").unwrap();
    let reference: RefOr<Schema> = RefOr::Reference { reference_path: "#/components/schemas/ConnectedAccountResponse".to_string() };
    _ = reference.resolve(&spec).unwrap();

    assert_eq!(
        &spec.schemas().len(),
        &spec.components.clone().map(|c| c.schemas.len()).unwrap_or(0),
    );
    assert_ne!(
        &spec.schemas().len(),
        &0
    );
}

#[test]
fn deserialize_bungie_v2_json() {
    let spec = from_path("../resources/bungie-spec-v2.json").unwrap();
    let reference: RefOr<Schema> = RefOr::Reference { reference_path: "#/definitions/Applications.Application".to_string() };
    _ = reference.resolve(&spec).unwrap();

    assert_eq!(
        &spec.schemas().len(),
        &spec.definitions.len(),
    );
    assert_ne!(
        &spec.schemas().len(),
        &0
    );
}

#[test]
fn deserialize_bungie_v3_json() {
    let spec = from_path("../resources/bungie-spec-v3.json").unwrap();
    let reference: RefOr<Schema> = RefOr::Reference { reference_path: "#/components/schemas/Exceptions.PlatformErrorCodes".to_string() };
    _ = reference.resolve(&spec).unwrap();

    assert_eq!(
        &spec.schemas().len(),
        &spec.components.clone().map(|c| c.schemas.len()).unwrap_or(0),
    );
    assert_ne!(
        &spec.schemas().len(),
        &0
    );
}
