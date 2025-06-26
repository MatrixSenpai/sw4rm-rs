use openapi_core::Spec;
use openapi_core::reference::*;

#[test]
fn test_external_reference() {
    let input = [
        ("description.yml#/introduction", "description.yml"),
        ("path/to/schema.yaml#/item/object", "path/to/schema.yaml"),
    ];

    for (input, matching) in input {
        let reference: Reference = input.to_string().try_into().unwrap();

        assert!(reference.source.is_some());
        assert_eq!(&reference.source.unwrap(), matching)
    }
}

#[test]
fn test_location() {
    let input = [
        (
            "description.yml#/introduction",
            ReferenceLocation::Unknown("introduction".to_string()),
        ),
        (
            "path/to/schema.yaml#/item/object",
            ReferenceLocation::Unknown("item".to_string()),
        ),
        ("#/components/schema/Hello", ReferenceLocation::Components),
        ("document.json#/servers/0", ReferenceLocation::Servers),
    ];

    let mut flag = true;
    for (input, matching) in input {
        let reference: Reference = match input.to_string().try_into() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Cannot parse {input}: {e:?}");
                flag = false;
                continue;
            }
        };

        assert_eq!(reference.location, matching)
    }

    assert!(flag);
}

#[test]
fn test_kind() {
    let input = [
        ("description.yml#/introduction", None),
        (
            "path/to/schema.yaml#/item/object",
            Some(ReferenceKind::Unknown("object".to_string())),
        ),
        ("#/components/examples/item", Some(ReferenceKind::Examples)),
        ("document.json#/servers/links", Some(ReferenceKind::Links)),
    ];

    let mut flag = true;
    for (input, matching) in input {
        let reference: Reference = match input.to_string().try_into() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Cannot parse {input}: {e:?}");
                flag = false;
                continue;
            }
        };

        assert_eq!(reference.kind, matching)
    }

    assert!(flag);
}

#[test]
fn test_name() {
    let input = [
        ("description.yml#/introduction", None),
        ("path/to/schema.yaml#/item/object", None),
        ("#/components/examples/item", Some("item".to_string())),
        ("document.json#/servers/links/0", Some("0".to_string())),
    ];

    let mut flag = true;
    for (input, matching) in input {
        let reference: Reference = match input.to_string().try_into() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Cannot parse {input}: {e:?}");
                flag = false;
                continue;
            }
        };

        assert_eq!(reference.name, matching)
    }

    assert!(flag);
}

#[test]
fn test_external_resolve() {
    let test_spec_file = std::fs::read_to_string("../test_specs/digitalocean.yaml").unwrap();
    let test_spec: Spec = serde_yaml::from_str(&test_spec_file).unwrap();

    let description = &test_spec.info.description;
    assert!(description.is_some());
    let description = description.clone().unwrap();

    let reference = match description {
        RefOr::Reference { location, .. } => location,
        RefOr::Object(_) => unreachable!("Unexpected object where expected reference"),
    };

    let value: Box<String> = reference
        .resolve_external(Some("../test_specs".to_string()))
        .unwrap();
    assert!(!value.is_empty());
}
