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
    test_spec("openapi_3_0.yaml", "openapi");
    test_spec("riot-swaggerspec-2.0.json", "riot");
    test_spec("bungie-spec.json", "bungie");
    test_spec("discord-spec.json", "discord");
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

fn test_spec<T: Into<String> + std::fmt::Display>(read_file: T, write_dir: T) {
    let spec = sw4rm_rs::from_path(format!("../resources/{read_file}")).unwrap();
    let object_count = &spec.schemas().len();
    let spec_name = spec.info.title.clone();
    let parsed = sw4rm_rs_generation::transform_spec(spec, None);
    let parsed_count = &parsed.len();
    let transformed = sw4rm_rs_generation::transform_files(parsed, None);

    std::fs::create_dir(format!("../resources/{write_dir}")).ok();

    for (name, file) in transformed.into_iter() {
        let contents = prettyplease::unparse(&file);

        std::fs::write(
            format!("../resources/{write_dir}/{name}.rs"),
            contents
        ).unwrap();
    }

    info!("{spec_name} spec: {parsed_count}/{object_count} parsed & written")
}
