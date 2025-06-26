use anyhow::Context;
use openapi_core::Spec;
use serde_json::from_str as json_str;
use serde_yaml::from_str as yaml_str;

const SKIP_FILE_NAMES: [&'static str; 1] = ["description.yml"];

#[test]
fn basic_decode() {
    let mut flag = true;
    for file in std::fs::read_dir("../test_specs").unwrap() {
        let file = file.unwrap();
        let path = file.path();
        let metadata = file.metadata().unwrap();

        if !metadata.is_file()
            || SKIP_FILE_NAMES.contains(&path.file_name().unwrap().to_str().unwrap())
        {
            continue;
        }
        let de: fn(&str) -> anyhow::Result<Spec> = match path.extension().and_then(|v| v.to_str()) {
            Some("json") => |v| json_str::<Spec>(v).context("json error"),
            Some("yaml") | Some("yml") => |v| yaml_str(v).context("yaml error"),
            _ => continue,
        };

        let file_contents = std::fs::read_to_string(&path).unwrap();
        let contents = match de(&file_contents) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Could not deserialize {:?}\n {e:?}", path);
                flag = false;
                continue;
            }
        };

        print!("Checking {:?}...", path.file_name().unwrap());
        assert!(contents.openapi.contains("3"));

        println!("ok");
    }

    assert!(flag);
}
