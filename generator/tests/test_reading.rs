use sw4rm_rs_generation::*;

#[cfg(test)]
#[ctor::ctor]
fn initialize() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init_timed()
}

#[test]
fn test_spec_parsing() {
    let openapi_spec = sw4rm_rs::from_path("../resources/openapi_3_0.yaml").unwrap();
    sw4rm_rs_generation::parsing::parse_spec(openapi_spec, None);
    
    let riot_spec = sw4rm_rs::from_path("../resources/riot-swaggerspec-2.0.json").unwrap();
    sw4rm_rs_generation::parsing::parse_spec(riot_spec, None);

    let bungie_spec = match sw4rm_rs::from_path("../resources/bungie-spec.json") {
        Ok(val) => val,
        Err(e) => {
            if let sw4rm_rs::error::Error::YamlError(e) = &e {
                println!("location: {:?}", e.location().unwrap());
            };
            panic!("{:?}", e);
        }
    };
    sw4rm_rs_generation::parsing::parse_spec(bungie_spec, None);

    let discord_spec = sw4rm_rs::from_path("../resources/discord-spec.json").unwrap();
    sw4rm_rs_generation::parsing::parse_spec(discord_spec, None);
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
