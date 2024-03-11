use proc_macro2::Ident;
use syn::{
    AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, PathSegment, Token,
    Type, TypePath,
    __private::Span,
    punctuated::Punctuated,
    token::PathSep,
};

use super::GenerationError;
use sw4rm_rs::{
    RefOr, Spec,
    shared::{
        Schema, SchemaType
    }
};

pub fn recursive_field_type(
    spec: &Spec,
    schema: &Schema,
    is_optional: bool,
    is_array: bool,
    field_title: Option<String>,
    field_type: Option<SchemaType>,
) -> Result<Punctuated<PathSegment, PathSep>, GenerationError> {
    if is_optional {
        let child_segments = recursive_field_type(
            spec, schema, false, is_array, field_title, field_type
        )?;

        let mut args: Vec<GenericArgument> = Vec::new();
        args.push(GenericArgument::Type(Type::Path(
            TypePath { qself: None, path: Path { leading_colon: None, segments: child_segments } }
        )));

        let generic_arguments = AngleBracketedGenericArguments {
            args: args.into_iter().collect(),
            colon2_token: None,
            lt_token: Token![<](Span::call_site()),
            gt_token: Token![>](Span::call_site()),
        };

        let ident = Ident::new("Option", Span::call_site());

        let mut final_segment: Vec<PathSegment> = Vec::new();
        final_segment.push(
            PathSegment { ident, arguments: PathArguments::AngleBracketed(generic_arguments) }
        );

        return Ok(
            final_segment.into_iter().collect()
        );
    }

    if is_array {
        let child_segments = recursive_field_type(
            spec, schema, is_optional, false, field_title, field_type
        )?;

        let mut args: Vec<GenericArgument> = Vec::new();
        args.push(GenericArgument::Type(Type::Path(
            TypePath { qself: None, path: Path { leading_colon: None, segments: child_segments } }
        )));

        let generic_arguments = AngleBracketedGenericArguments {
            args: args.into_iter().collect(),
            colon2_token: None,
            lt_token: Token![<](Span::call_site()),
            gt_token: Token![>](Span::call_site()),
        };

        let ident = Ident::new("Vec", Span::call_site());

        let mut final_segment: Vec<PathSegment> = Vec::new();
        final_segment.push(
            PathSegment { ident, arguments: PathArguments::AngleBracketed(generic_arguments) }
        );

        return Ok(
            final_segment.into_iter().collect()
        );
    }

    // TODO: this whole block needs to be optimized
    if schema.schema_type.eq(&Some(SchemaType::Object)) && (
        schema.title.is_none() || schema.clone().title.is_some_and(|t| t.is_empty())
    ) {
        let mut string_arguments: Vec<PathSegment> = Vec::new();
        string_arguments.push(PathSegment {
            ident: Ident::new("String", Span::call_site()),
            arguments: PathArguments::None,
        });

        let mut json_arguments: Vec<PathSegment> = Vec::new();
        json_arguments.push(PathSegment {
            ident: Ident::new("serde_json", Span::call_site()),
            arguments: PathArguments::None,
        });
        json_arguments.push(PathSegment {
            ident: Ident::new("Value", Span::call_site()),
            arguments: PathArguments::None,
        });

        let mut generic_args: Vec<GenericArgument> = Vec::new();
        generic_args.push(GenericArgument::Type(Type::Path(
            TypePath { qself: None, path: Path { leading_colon: None, segments: string_arguments.into_iter().collect() } }
        )));
        generic_args.push(GenericArgument::Type(Type::Path(
            TypePath { qself: None, path: Path { leading_colon: None, segments: json_arguments.into_iter().collect() } }
        )));

        let generic_arguments = AngleBracketedGenericArguments {
            args: generic_args.into_iter().collect(),
            colon2_token: None,
            lt_token: Token![<](Span::call_site()),
            gt_token: Token![>](Span::call_site()),
        };

        let ident = Ident::new("HashMap", Span::call_site());

        let mut final_segment: Vec<PathSegment> = Vec::new();
        final_segment.push(
            PathSegment {
                ident,
                arguments: PathArguments::AngleBracketed(generic_arguments)
            }
        );

        return Ok(
            final_segment.into_iter().collect()
        );
    }

    let ident_type = get_type(
        spec, field_title, field_type, &schema.items, &schema.additional_properties
    )?;

    let mut final_segment: Vec<PathSegment> = Vec::new();
    final_segment.push(
        PathSegment {
            ident: Ident::new(ident_type.as_str(), Span::call_site()),
            arguments: PathArguments::None,
        }
    );

    Ok(
        final_segment.into_iter().collect()
    )
}

fn get_type(
    spec: &Spec,
    field_title: Option<String>,
    field_type: Option<SchemaType>,
    properties: &Option<RefOr<Box<Schema>>>,
    additional_properties: &Option<RefOr<Box<Schema>>>,
) -> Result<String, GenerationError> {
    match field_type {
        Some(SchemaType::Boolean) => return Ok("bool".to_string()),
        Some(SchemaType::Integer) => return Ok("i64".to_string()),
        Some(SchemaType::Number) => return Ok("f64".to_string()),
        Some(SchemaType::String) => return Ok("String".to_string()),
        Some(SchemaType::Object) if field_title.is_some() => return Ok(field_title.unwrap()),

        _ => ()
    };

    if let Some(items) = properties {
        let items = items.resolve(spec)?;
        return get_type(spec, items.title, items.schema_type, &items.items, &items.additional_properties);
    }

    if let Some(additional_items) = additional_properties {
        let items = additional_items.resolve(spec)?;
        return get_type(spec, items.title, items.schema_type, &items.items, &items.additional_properties);
    }

    unreachable!()
}
