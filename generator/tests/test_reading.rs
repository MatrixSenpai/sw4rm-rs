use log::info;
// use sw4rm_rs_generation::*;

#[cfg(test)]
#[ctor::ctor]
fn initialize() {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init_timed();
}

#[test]
fn test_spec_parsing() {
    let openapi_spec = sw4rm_rs::from_path("../resources/openapi_3_0.yaml").unwrap();
    let openapi_object_count = &openapi_spec.schemas().len();
    let openapi_parsed = sw4rm_rs_generation::parsing::transform_spec(openapi_spec, None);
    
    let riot_spec = sw4rm_rs::from_path("../resources/riot-swaggerspec-2.0.json").unwrap();
    let riot_object_count = &riot_spec.schemas().len();
    let riot_parsed = sw4rm_rs_generation::parsing::transform_spec(riot_spec, None);

    let bungie_spec = sw4rm_rs::from_path("../resources/bungie-spec.json").unwrap();
    let bungie_object_count = &bungie_spec.schemas().len();
    let bungie_parsed = sw4rm_rs_generation::parsing::transform_spec(bungie_spec, None);

    let discord_spec = sw4rm_rs::from_path("../resources/discord-spec.json").unwrap();
    let discord_object_count = &discord_spec.schemas().len();
    let discord_parsed = sw4rm_rs_generation::parsing::transform_spec(discord_spec, None);

    info!("OpenAPI spec: {}/{openapi_object_count} parsed", openapi_parsed.len());
    info!("Riot spec: {}/{riot_object_count} parsed", riot_parsed.len());
    info!("Bungie spec: {}/{bungie_object_count} parsed", bungie_parsed.len());
    info!("Discord spec: {}/{discord_object_count} parsed", discord_parsed.len());
}
//
// #[test]
// fn read_swagger_v2_yaml() {
//     _ = parse_file("../resources/swagger_2_0.yaml").unwrap();
// }
//
// #[test]
// fn read_swagger_v2_json() {
//     _ = parse_file("../resources/riot-swaggerspec-2.0.json").unwrap();
// }
//
// #[test]
// fn read_openapi_v3_yaml() {
//     _ = parse_file("../resources/openapi_3_0.yaml").unwrap();
// }
//
// #[test]
// fn read_openapi_v3_json() {
//     _ = parse_file("../resources/riot-openapi-3.0.0.json").unwrap();
// }
