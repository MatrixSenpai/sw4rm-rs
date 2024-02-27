#![allow(dead_code, unused)]

use std::collections::HashMap;
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::ToTokens;
use regex::{Captures, Regex};
use syn::*;
use syn::__private::Span;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Bracket, Comma, Paren, PathSep, Pub, Struct};

use crate::models::{
    *, shared::*, openapi_v2::*, openapi_v3_0::*,
};

lazy_static! {
    static ref HAS_NUMBER_REGEX: Regex = Regex::new(
        r"\d+",
    ).unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(
        r"^(?<number>\d+)",
    ).unwrap();
    static ref TYPE_REGEX: Regex = Regex::new(
        r"^(?<item_type>type)$",
    ).unwrap();
    static ref MATCH_REGEX: Regex = Regex::new(
        r"^(?<item_match>match)$",
    ).unwrap();
}

#[derive(Debug)]
pub enum GenerationError {

}

pub fn parse_spec(spec: Spec) -> std::result::Result<HashMap<String, File>, GenerationError> {
    let models = spec.schemas();
    let models_resolved = models.iter()
        .map(|(file_name, schema_item)| (file_name, schema_item.resolve(&spec).unwrap()))
        .collect();

    let files = parse_files(&spec, models_resolved);
    Ok(files)
}

pub fn parse_files(
    spec: &Spec,
    file_items: HashMap<&String, Schema>,
) -> HashMap<String, File> {
    file_items.into_iter()
        .map(|(file_name, schema)| (file_name.clone(), parse_file(spec, &file_name, schema)))
        .collect()
}

pub fn parse_file(
    spec: &Spec,
    file_name: &String,
    schema: Schema,
) -> File {
    let use_hashmap_item = ItemUse {
        attrs: Vec::default(),
        vis: Visibility::Inherited,
        use_token: Token![use](Span::call_site()),
        leading_colon: None,
        semi_token: Token![;](Span::call_site()),
        tree: UseTree::Path(UsePath {
            ident: Ident::new("std", Span::call_site()),
            colon2_token: Token![::](Span::call_site()),
            tree: Box::new(UseTree::Path(UsePath {
                ident: Ident::new("collections", Span::call_site()),
                colon2_token: Token![::](Span::call_site()),
                tree: Box::new(UseTree::Name(UseName {
                    ident: Ident::new("HashMap", Span::call_site())
                }))
            }))
        })
    };

    let items = vec![Item::Use(use_hashmap_item), parse_item(spec, file_name, schema)];

    File {
        items,
        attrs: Vec::default(),
        shebang: None,
    }
}

pub fn parse_item(
    spec: &Spec,
    file_name: &String,
    schema: Schema,
) -> Item {
    let ident = Ident::new(schema.title.clone().unwrap_or(file_name.clone()).as_str(), Span::call_site());
    let attrs = parse_struct_attrs(schema.description.clone());
    let fields = FieldsNamed {
        named: parse_struct_fields(spec, &schema),
        brace_token: Brace::default(),
    };

    Item::Struct(ItemStruct {
        ident, attrs,
        fields: Fields::Named(fields),
        vis: Visibility::Public(Token![pub](Span::call_site())),
        semi_token: None,
        struct_token: Token![struct](Span::call_site()),
        generics: Generics::default(),
    })
}

pub fn parse_struct_attrs(
    description: Option<String>,
) -> Vec<Attribute> {
    let mut attrs = Vec::new();

    if let Some(description) = description {
        let description = format!(" {description}");

        let mut comment_segments = Punctuated::new();
        comment_segments.push(PathSegment { ident: Ident::new("doc", Span::call_site()), arguments: PathArguments::None });
        let comment_expression = ExprLit {
            attrs: Vec::default(),
            lit: Lit::Str(LitStr::new(description.as_str(), Span::call_site()))
        };
        let comment_meta = MetaNameValue {
            path: Path { leading_colon: None, segments: comment_segments },
            eq_token: Token![=](Span::call_site()),
            value: Expr::Lit(comment_expression),
        };
        let comment_attr = Attribute {
            pound_token: Token![#](Span::call_site()),
            style: AttrStyle::Outer,
            bracket_token: Bracket::default(),
            meta: Meta::NameValue(comment_meta)
        };
        attrs.push(comment_attr);
    }

    let mut derive_segments = Punctuated::new();
    derive_segments.push(PathSegment { ident: Ident::new("derive", Span::call_site()), arguments: PathArguments::None });
    let derive_token_stream = quote::quote! {
        Debug, Serialize, Deserialize, Default, Clone, PartialEq
    };
    let derive_meta = MetaList {
        path: Path { leading_colon: None, segments: derive_segments },
        delimiter: MacroDelimiter::Paren(Paren::default()),
        tokens: derive_token_stream,
    };
    let derive_attr = Attribute {
        pound_token: Token![#](Span::call_site()),
        style: AttrStyle::Outer,
        bracket_token: Bracket::default(),
        meta: Meta::List(derive_meta),
    };
    attrs.push(derive_attr);

    let mut default_segments = Punctuated::new();
    default_segments.push(PathSegment { ident: Ident::new("default", Span::call_site()), arguments: PathArguments::None });
    let default_meta = Path {
        leading_colon: None,
        segments: default_segments,
    };

    let mut rename_segments = Punctuated::new();
    rename_segments.push(PathSegment { ident: Ident::new("rename_all", Span::call_site()), arguments: PathArguments::None });
    let rename_expression = ExprLit {
        attrs: Vec::default(),
        lit: Lit::Str(LitStr::new("camelCase", Span::call_site()))
    };
    let rename_meta = MetaNameValue {
        path: Path { leading_colon: None, segments: rename_segments },
        eq_token: Token![=](Span::call_site()),
        value: Expr::Lit(rename_expression)
    };

    let mut serde_attr_segments: Punctuated<TokenStream, Comma> = Punctuated::new();
    serde_attr_segments.push(default_meta.to_token_stream());
    serde_attr_segments.push(rename_meta.to_token_stream());

    let mut serde_segments = Punctuated::new();
    serde_segments.push(PathSegment { ident: Ident::new("serde", Span::call_site()), arguments: PathArguments::None });
    let serde_meta = MetaList {
        path: Path { leading_colon: None, segments: serde_segments },
        delimiter: MacroDelimiter::Paren(Paren::default()),
        tokens: serde_attr_segments.to_token_stream()
    };
    let serde_attr = Attribute {
        pound_token: Token![#](Span::call_site()),
        style: AttrStyle::Outer,
        bracket_token: Bracket::default(),
        meta: Meta::List(serde_meta),
    };

    attrs.push(serde_attr);

    attrs
}

pub fn parse_struct_fields(
    spec: &Spec,
    schema: &Schema,
) -> Punctuated<Field, Comma> {
    schema.properties.iter()
        .map(|(object_name, object_reference)| (object_name, object_reference.resolve(spec).unwrap()))
        .map(|(object_name, schema)| parse_struct_field(spec, object_name, schema))
        .collect::<Punctuated<_, Comma>>()
}

pub fn parse_struct_field(
    spec: &Spec,
    property_name: &String,
    schema: Box<Schema>,
) -> Field {
    let binding = schema.clone();
    let schema_ref = binding.as_ref();

    let (ident, needs_rename) = parse_field_ident(property_name);
    let attrs = parse_field_attrs(&schema.description, property_name, needs_rename);

    let required_fields = schema.required.clone();

    let path_segments = recursive_field_type(
        spec,
        schema_ref,
        !required_fields.contains(&property_name),
        schema.schema_type.eq(&Some(SchemaType::Array)),
        schema.title,
        schema.schema_type
    );
    let path = TypePath {
        qself: None,
        path: Path { leading_colon: None, segments: path_segments }
    };

    Field {
        attrs,
        ident: Some(ident),
        vis: Visibility::Public(Token![pub](Span::call_site())),
        mutability: FieldMutability::None,
        colon_token: None,
        ty: Type::Path(path),
    }
}

pub fn parse_field_ident(
    item_name: &String,
) -> (Ident, bool) {
    let cased = item_name.clone().to_case(Case::Snake);
    let number_checked = NUMBER_REGEX.replace(&cased.as_str(), |cap: &Captures| {
        let number = cap["number"].parse::<i64>().unwrap();
        rust_numerals::number_to_cardinal(number)
    });
    let type_checked = TYPE_REGEX.replace(&number_checked, "kind");
    let match_checked = MATCH_REGEX.replace(&type_checked, "match_item");

    let ident = Ident::new(&match_checked, Span::call_site());
    let needs_rename = cased.ne(&match_checked) || HAS_NUMBER_REGEX.is_match(item_name);

    (ident, needs_rename)
}

pub fn parse_field_attrs(
    description: &Option<String>,
    field_name: &String,
    needs_rename: bool,
) -> Vec<Attribute> {
    let mut attrs = Vec::new();

    if let Some(description) = description {
        let description = format!(" {description}");

        let mut comment_segments = Punctuated::new();
        comment_segments.push(PathSegment { ident: Ident::new("doc", Span::call_site()), arguments: PathArguments::None });
        let comment_expression = ExprLit {
            attrs: Vec::default(),
            lit: Lit::Str(LitStr::new(description.as_str(), Span::call_site()))
        };
        let comment_meta = MetaNameValue {
            path: Path { leading_colon: None, segments: comment_segments },
            eq_token: Token![=](Span::call_site()),
            value: Expr::Lit(comment_expression),
        };
        let comment_attr = Attribute {
            pound_token: Token![#](Span::call_site()),
            style: AttrStyle::Outer,
            bracket_token: Bracket::default(),
            meta: Meta::NameValue(comment_meta)
        };
        attrs.push(comment_attr);
    }

    if needs_rename {
        let mut rename_segments = Punctuated::new();
        rename_segments.push(PathSegment { ident: Ident::new("rename", Span::call_site()), arguments: PathArguments::None });
        let rename_expression = ExprLit {
            attrs: Vec::default(),
            lit: Lit::Str(LitStr::new(field_name, Span::call_site()))
        };
        let rename_meta = MetaNameValue {
            path: Path { leading_colon: None, segments: rename_segments },
            eq_token: Token![=](Span::call_site()),
            value: Expr::Lit(rename_expression)
        };
        let mut serde_segments = Punctuated::new();
        serde_segments.push(PathSegment { ident: Ident::new("serde", Span::call_site()), arguments: PathArguments::None });
        let serde_meta = MetaList {
            path: Path { leading_colon: None, segments: serde_segments },
            delimiter: MacroDelimiter::Paren(Paren::default()),
            tokens: rename_meta.to_token_stream()
        };
        let serde_attr = Attribute {
            pound_token: Token![#](Span::call_site()),
            style: AttrStyle::Outer,
            bracket_token: Bracket::default(),
            meta: Meta::List(serde_meta),
        };
        attrs.push(serde_attr);
    }

    attrs
}

fn recursive_field_type(
    spec: &Spec,
    schema: &Schema,
    is_optional: bool,
    is_array: bool,
    field_title: Option<String>,
    field_type: Option<SchemaType>,
) -> Punctuated<PathSegment, PathSep> {
    if is_optional {
        let child_segments = recursive_field_type(spec, schema, false, is_array, field_title, field_type);

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

        return final_segment.into_iter().collect();
    }

    if is_array {
        let child_segments = recursive_field_type(spec, schema, is_optional, false, field_title, field_type);

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

        return final_segment.into_iter().collect();
    }

    // TODO: this whole block needs to be optimized
    if schema.schema_type.eq(&Some(SchemaType::Object)) && (schema.title.is_none() || schema.clone().title.is_some_and(|t| t.is_empty())) {
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
            PathSegment { ident, arguments: PathArguments::AngleBracketed(generic_arguments) }
        );

        return final_segment.into_iter().collect();
    }

    let ident_type = get_type(spec, field_title, field_type, &schema.items, &schema.additional_properties);

    let mut final_segment: Vec<PathSegment> = Vec::new();
    final_segment.push(
        PathSegment {
            ident: Ident::new(ident_type.as_str(), Span::call_site()),
            arguments: PathArguments::None,
        }
    );

    final_segment.into_iter().collect()
}

fn get_type(
    spec: &Spec,
    field_title: Option<String>,
    field_type: Option<SchemaType>,
    properties: &Option<RefOr<Box<Schema>>>,
    additional_properties: &Option<RefOr<Box<Schema>>>,
) -> String {
    match field_type {
        Some(SchemaType::Boolean) => return "bool".to_string(),
        Some(SchemaType::Integer) => return "i64".to_string(),
        Some(SchemaType::Number) => return "f64".to_string(),
        Some(SchemaType::String) => return "String".to_string(),
        Some(SchemaType::Object) if field_title.is_some() => return field_title.unwrap(),

        _ => ()
    };

    if let Some(items) = properties {
        let items = items.resolve(spec).unwrap();
        return get_type(spec, items.title, items.schema_type, &items.items, &items.additional_properties);
    }

    if let Some(additional_items) = additional_properties {
        let items = additional_items.resolve(spec).unwrap();
        return get_type(spec, items.title, items.schema_type, &items.items, &items.additional_properties);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parsing() {
        let spec = crate::from_path("./resources/riot-openapi-3.0.0.json").unwrap();
        let files = super::parse_spec(spec).unwrap();

        for (_, file) in files {
            let file_str = prettyplease::unparse(&file);
            println!("{file_str}")
        }
    }
}