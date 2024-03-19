use proc_macro2::Span;
use quote::quote;
use syn::{punctuated::Punctuated, token::PathSep, PathSegment, Token};

use super::{error::GenerationError, identifier::Identifier};


/// An attribute to be applied. E.g. `#[derive(Debug)]`, `#[command]`
#[derive(Debug, Clone)]
pub enum Attribute {
    /// A single attribute. E.g. `#[command]`
    Single(Identifier),
    /// A parenthesized attribute. E.g. `#[derive(Debug)]`
    Paren(ParenAttribute),
}

impl Attribute {
    pub fn default_struct_derive_attrs(additional_addrs: Option<Vec<Self>>) -> Vec<Self> {
        let mut attrs = additional_addrs.unwrap_or_default();
        let attr_item = vec![
            "Debug",
            "Serialize",
            "Deserialize",
            "Clone",
            "PartialEq",
        ].into_iter()
            .map(String::from)
            .map(Identifier::new)
            .map(ParenAttributeItem::Single)
            .collect();

        attrs.push(
            Attribute::Paren(
                ParenAttribute { name: Identifier::new("derive".to_string()), items: attr_item }
            )
        );

        attrs
    }

    pub fn default_optional_derive_attrs(additional_addrs: Option<Vec<Self>>) -> Vec<Self> {
        let mut attrs = additional_addrs.unwrap_or_default();
        let attr_item = vec![
            "Debug",
            "Serialize",
            "Deserialize",
            "Copy",
            "Clone",
            "PartialEq",
            "Eq",
            "PartialOrd",
            "Ord",
        ].into_iter()
            .map(String::from)
            .map(Identifier::new)
            .map(ParenAttributeItem::Single)
            .collect();

        attrs.push(
            Attribute::Paren(
                ParenAttribute { name: Identifier::new("derive".to_string()), items: attr_item }
            )
        );

        attrs
    }
}

impl From<Attribute> for syn::Attribute {
    fn from(val: Attribute) -> Self {
        let item = match val {
            Attribute::Paren(attribute) => attribute.into(),
            Attribute::Single(identifier) => {
                let mut segments: Punctuated<PathSegment, PathSep> = Punctuated::new();

                let attr_ident = identifier.field_rep(false).0;
                segments.push(syn::PathSegment {
                    ident: syn::Ident::new(attr_ident.as_str(), Span::call_site()),
                    arguments: syn::PathArguments::None,
                });

                syn::Meta::Path(
                    syn::Path { leading_colon: None, segments }
                )
            },
        };

        syn::Attribute {
            pound_token: Token![#](Span::call_site()),
            style: syn::AttrStyle::Outer,
            bracket_token: syn::token::Bracket::default(),
            meta: item,
        }
    }
}

/// A parenthesized attribute. E.g. `#[derive(Debug)]`
#[derive(Debug, Clone)]
pub struct ParenAttribute {
    /// The name outside parenthesis. The `derive` in `#[derive(Debug)]`
    pub name: Identifier,
    /// The items inside the parenthesis. The `Debug` in `#[derive(Debug)]`
    pub items: Vec<ParenAttributeItem>,
}

/// A parenthesized attribute. E.g. `Debug`, `rename_all = "foo"`
#[derive(Debug, Clone, PartialEq)]
pub enum ParenAttributeItem {
    /// A single item. E.g. `Debug`
    Single(Identifier),
    /// An assigned value. E.g. `rename_all = "foo"`
    AssignValue(Identifier, Identifier),
    /// A nested item. E.g. `serde(rename(serialize = "ser_name"))
    Nested(Identifier, Box<ParenAttributeItem>)
}

impl From<ParenAttribute> for syn::Meta {
    fn from(val: ParenAttribute) -> Self {
        let attr_ident = val.name.field_rep(false).0;
        let mut attr_segments: syn::punctuated::Punctuated<syn::PathSegment, PathSep> = syn::punctuated::Punctuated::new();
        attr_segments.push(syn::PathSegment {
            ident: syn::Ident::new(attr_ident.as_str(), Span::call_site()),
            arguments: syn::PathArguments::None,
        });
        let attr_path = syn::Path { leading_colon: None, segments: attr_segments };

        let token_list = val.items.iter()
            .map(|item| {
                fn handle_item(item: &ParenAttributeItem) -> proc_macro2::TokenStream {
                    match item {
                        ParenAttributeItem::Single(ident) => {
                            let ident_name = ident.clone().trait_rep(false).0;
                            let ident = syn::Ident::new(ident_name.as_str(), Span::call_site());
                            quote! { #ident }
                        },
                        ParenAttributeItem::AssignValue(left, right) => {
                            let left_ident_name = left.clone().trait_rep(false).0;
                            let left_ident = syn::Ident::new(left_ident_name.as_str(), Span::call_site());

                            let right = right.raw_value();

                            quote! { #left_ident = "#right" }
                        },
                        ParenAttributeItem::Nested(ident, inner) => {
                            let ident_name = ident.clone().trait_rep(false).0;
                            let ident = syn::Ident::new(ident_name.as_str(), Span::call_site());

                            let inner = handle_item(inner);

                            quote! { #ident(#inner) }
                        },
                    }
                };
                handle_item(item)
            })
            .collect::<Vec<_>>();

        syn::Meta::List(syn::MetaList {
            path: attr_path,
            delimiter: syn::MacroDelimiter::Paren(syn::token::Paren::default()),
            tokens: quote! {
               #(#token_list), *
            }
        })
    }
}
