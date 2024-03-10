use syn::{
    Field, FieldMutability, Path, Token, Type, TypePath, Visibility,
    __private::Span,
    punctuated::Punctuated,
    token::Comma,
};

use super::{
    GenerationError,
    ident::parse_field_ident,
    attribute::parse_field_attrs,
    typed::recursive_field_type,
};
use sw4rm_rs::{
    Spec,
    shared::{Schema, SchemaType}
};

/// Returns a set of fields inside a struct. Includes rename attributes and doc comments, if
/// available. Also attempts to find any invalid identifiers and rename them
pub fn parse_struct_fields(
    spec: &Spec,
    schema: &Schema,
) -> Result<Punctuated<Field, Comma>, GenerationError> {
    Ok(
        schema.properties.iter()
            .map(|(object_name, object_reference)| (object_name, object_reference.resolve(spec).unwrap()))
            .map(|(object_name, schema)| parse_struct_field(spec, object_name, schema).unwrap())
            .collect::<Punctuated<_, Comma>>()
    )
}

/// Returns a single struct field. Includes doc and rename, if required.
fn parse_struct_field(
    spec: &Spec,
    property_name: &String,
    schema: Box<Schema>,
) -> Result<Field, GenerationError> {
    let binding = schema.clone();
    let schema_ref = binding.as_ref();

    let (ident, needs_rename) = parse_field_ident(property_name);
    let attrs = parse_field_attrs(&schema.description, property_name, needs_rename);

    let required_fields = schema.required.clone();

    let path_segments = recursive_field_type(
        spec,
        schema_ref,
        !required_fields.contains(&property_name),
        schema.schema_type.eq(&Some(sw4rm_rs::shared::SchemaTypeContainer::SingleType(SchemaType::Array))),
        schema.title,
        schema.schema_type.map(|s| {
            match s {
                sw4rm_rs::shared::SchemaTypeContainer::SingleType(v) => v,
                sw4rm_rs::shared::SchemaTypeContainer::MultiType(v) => v.first().unwrap().clone(),
            }
        })
    )?;
    let path = TypePath {
        qself: None,
        path: Path { leading_colon: None, segments: path_segments }
    };

    Ok(
        Field {
            attrs,
            ident: Some(ident),
            vis: Visibility::Public(Token![pub](Span::call_site())),
            mutability: FieldMutability::None,
            colon_token: None,
            ty: Type::Path(path),
        }
    )
}
