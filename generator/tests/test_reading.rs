use sw4rm_rs_generation::*;

#[test]
fn read_swagger_v2_yaml() {
    _ = parse_file("../resources/swagger_2_0.yaml").unwrap();
}

#[test]
fn read_swagger_v2_json() {
    _ = parse_file("../resources/riot-swaggerspec-2.0.json").unwrap();
}

#[test]
fn read_openapi_v3_yaml() {
    _ = parse_file("../resources/openapi_3_0.yaml").unwrap();
}

#[test]
fn read_openapi_v3_json() {
    _ = parse_file("../resources/riot-openapi-3.0.0.json").unwrap();
}