use std::sync::Arc;
use std::collections::HashMap;
use proc_macro2::Span;
use sw4rm_rs::{
    shared::{Schema, SchemaType, SchemaTypeContainer}, Spec
};
use syn::Token;
use super::{
    attribute::Attribute, error::GenerationError, identifier::Identifier, import::Import
};

/// A field in a struct or enum
#[derive(Debug, Clone)]
pub struct Field {
    /// If the type needs to be exported, it is easier to define it here than trying to pass it all
    /// the way up. Should be collected by `Model`
    pub imports: Vec<Import>,
    /// A documentation attribute
    pub documentation: Option<String>,
    /// Any attributes for the field. E.g. `serde(skip_serializing)`
    pub attributes: Vec<Attribute>,
    /// The field name. Should be `Ident` ready
    pub field_name: Identifier,
    /// The field type
    pub field_type: FieldType,
}

#[derive(Debug, Clone)]
pub struct FieldInputParams {
    pub definition_key: String,
    pub field: Box<Schema>,
    pub spec: Arc<Spec>,
}

impl TryFrom<FieldInputParams> for Field {
    type Error = GenerationError;

    fn try_from(value: FieldInputParams) -> Result<Self, Self::Error> {
        let field_name = value.field.title.clone()
            .unwrap_or(value.definition_key.clone());

        Ok(
            Self {
                // TODO: do imports
                imports: Vec::new(),
                documentation: value.field.description.clone(),
                // TODO: do attributes
                attributes: Vec::new(),
                field_name: field_name.into(),
                field_type: value.clone().try_into()?,
            }
        )
    }
}

impl From<Field> for syn::Field {
    fn from(val: Field) -> Self {
       let (ident, needs_rename) = val.field_name.field_rep(true); 

        syn::Field {
            attrs: Vec::new(),
            vis: syn::Visibility::Public(Token![pub](Span::call_site())),
            mutability: syn::FieldMutability::None,
            ident: Some(syn::Ident::new(ident.as_str(), Span::call_site())),
            colon_token: Some(Token![:](Span::call_site())),
            ty: val.field_type.into(),
        }
    }
}

/// Either a concrete or generic type
#[derive(Debug, Clone)]
pub enum FieldType {
    /// A concrete type. E.g. `String`, `impl DeserializeOwned`
    Concrete(FieldConcreteType),
    /// A generic field type. E.g. `Option<String>`
    Generic(FieldConcreteType, Box<FieldType>),
    /// A generic field type that specifies a key and value. E.g. `HashMap<String, String>`
    GenericPair(FieldConcreteType, Box<FieldType>, Box<FieldType>)
}

impl From<FieldType> for syn::Type {
    fn from(value: FieldType) -> Self {
        let mut segments: syn::punctuated::Punctuated<syn::PathSegment, _> = syn::punctuated::Punctuated::new();
        let segment = match value.clone() {
            FieldType::Concrete(t) => syn::PathSegment { ident: t.into(), arguments: syn::PathArguments::None },
            FieldType::Generic(t, _) => syn::PathSegment { ident: t.into(), arguments: value.into() },
            FieldType::GenericPair(t, _, _) => syn::PathSegment { ident: t.into(), arguments: value.into() },
        };
        segments.push(segment);
        
        syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path { leading_colon: None, segments },
        })
    }
}

impl From<FieldType> for syn::PathArguments {
    fn from(value: FieldType) -> Self {
        let mut generics: syn::punctuated::Punctuated<syn::GenericArgument, _> = syn::punctuated::Punctuated::new();
        match value {
            FieldType::Generic(_, i) => {
                let item_type = (*i).into();
                generics.push(syn::GenericArgument::Type(item_type));
            },
            FieldType::GenericPair(_, l, r) => {
                let left_type = (*l).into();
                generics.push(syn::GenericArgument::Type(left_type));

                let right_type = (*r).into();
                generics.push(syn::GenericArgument::Type(right_type));
            }
            _ => unreachable!(),
        };

        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Token![<](Span::call_site()),
            gt_token: Token![>](Span::call_site()),
            args: generics,
        })
    }
}

#[derive(Debug, Clone)]
pub enum FieldConcreteType {
    Array,
    Map,
    Optional,
    Bool,
    Integer,
    Float,
    String,
}

impl TryFrom<FieldInputParams> for FieldType {
    type Error = GenerationError;

    fn try_from(value: FieldInputParams) -> Result<Self, Self::Error> {
        let kind = match value.field.schema_type {
            Some(SchemaTypeContainer::SingleType(v)) => v,
            Some(SchemaTypeContainer::MultiType(v)) => v.first().unwrap().to_owned(),
            _ => return Err(GenerationError::MissingType)
        };

        let kind = match kind {
            SchemaType::Boolean => FieldConcreteType::Bool,
            SchemaType::String => FieldConcreteType::String,
            SchemaType::Integer => FieldConcreteType::Integer,
            SchemaType::Number => FieldConcreteType::Float,

            _ => return Err(Self::Error::Incomplete),
        };
        
        Ok(FieldType::Concrete(kind))
    }
}

impl From<FieldConcreteType> for syn::Ident {
    fn from(value: FieldConcreteType) -> Self {
        match value {
            FieldConcreteType::Array => syn::Ident::new("Vec", Span::call_site()),
            // TODO: report this as needing import...
            FieldConcreteType::Map => syn::Ident::new("HashMap", Span::call_site()),
            FieldConcreteType::Optional => syn::Ident::new("Option", Span::call_site()),
            FieldConcreteType::Bool => syn::Ident::new("bool", Span::call_site()),
            // TODO: this should be optimized...
            FieldConcreteType::Integer => syn::Ident::new("i64", Span::call_site()),
            // TODO: this should be optimized...
            FieldConcreteType::Float => syn::Ident::new("f64", Span::call_site()),
            FieldConcreteType::String => syn::Ident::new("String", Span::call_site()),
        }
    }
}
