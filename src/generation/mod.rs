#![allow(dead_code, unused)]

use std::collections::HashMap;
use proc_macro2::TokenStream;
use syn::*;
use syn::__private::Span;
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Paren, PathSep, Pub, Struct};

use crate::models::{
    *, shared::*, openapi_v2::*, openapi_v3_0::*,
};

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
    let items = vec![parse_item(spec, file_name, schema)];

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
    let ident = Ident::new(schema.title.unwrap_or(file_name.clone()).as_str(), Span::call_site());
    let attrs = parse_struct_attrs(schema.description);
    let fields = Fields::Unit;

    Item::Struct(ItemStruct {
        ident, attrs, fields,
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

    attrs
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