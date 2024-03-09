use sw4rm_rs::Reference;

#[test]
fn test_regex_captures_swagger_yaml() {
    let hay = "#/parameters/nested_response";
    _ = Reference::try_from(hay.to_string()).unwrap();
}
#[test]
fn test_regex_captures_swagger_json() {
    let hay = "#/definitions/account-v1.AccountDto";
    _ = Reference::try_from(hay.to_string()).unwrap();
}
#[test]
fn test_regex_captures_openapi_yaml() {
    let hay = "#/components/schemas/Pets";
    _ = Reference::try_from(hay.to_string()).unwrap();
}
#[test]
fn test_regex_captures_openapi_json() {
    let hay = "#/components/schemas/account-v1.AccountDto";
    _ = Reference::try_from(hay.to_string()).unwrap();
}
