use proc_macro2::Span;
use syn::Token;

use super::{
    attribute::Attribute, error::GenerationError, field::{Field, FieldInputParams}, file::FileInputParams, identifier::Identifier, import::Import
};

/// A struct representation
#[derive(Debug, Clone)]
pub struct Model {
    /// All imports that this model uses. Can be from another crate or part of this one
    pub imports: Vec<Import>,
    /// The documentation string that gets added to the model
    pub documentation: Option<String>,
    /// All attributes to be added to this model. E.g. derive, serde(rename = "foo"), serde(default)
    pub attributes: Vec<Attribute>,
    /// The name of the modifier. Should be `Ident` ready
    pub model_name: Identifier,
    /// Model fields
    pub fields: Vec<Field>,
    // TODO: this will be mildly more complex
    /// All manual implementations for the model. E.g. impl Deserialize for Foo
    pub implementations: Vec<()>,
}

impl TryInto<syn::Item> for Model {
    type Error = GenerationError;

    fn try_into(self) -> Result<syn::Item, Self::Error> {
        let attrs = self.attributes.into_iter()
            .map(syn::Attribute::from)
            .collect();

        let fields = self.fields.into_iter()
            .map(syn::Field::from)
            .collect::<syn::punctuated::Punctuated<_, syn::token::Comma>>();
        let fields_named = syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: fields,
        };


        let ident = self.model_name.trait_rep(false).0;

        Ok(syn::Item::Struct(syn::ItemStruct {
            attrs,
            vis: syn::Visibility::Public(Token![pub](Span::call_site())),
            struct_token: Token![struct](Span::call_site()),
            ident: syn::Ident::new(ident.as_str(), Span::call_site()),
            generics: syn::Generics { lt_token: None, gt_token: None, params: syn::punctuated::Punctuated::new(), where_clause: None, },
            fields: syn::Fields::Named(fields_named),
            semi_token: None,
        }))
    }
}

impl TryFrom<FileInputParams> for Model {
    type Error = GenerationError;

    fn try_from(value: FileInputParams) -> Result<Self, Self::Error> {
        let model_name = value.schema_item.title.unwrap_or(value.definition_key).into();

        let fields = value.schema_item.properties.iter()
            .flat_map(|(key, reference)| super::utils::resolve_boxed(value.spec.clone(), key, reference))
            .map(|(key, reference)| FieldInputParams { definition_key: key, field: reference, spec: value.spec.clone() })
            .flat_map(|params| params.try_into().ok())
            .collect();

        Ok(
            Self {
                // TODO: do imports
                imports: Vec::new(),
                documentation: value.schema_item.description,
                // TODO: do attributes
                attributes: Attribute::default_struct_derive_attrs(None),
                model_name,
                fields,
                // TODO: do implementations
                implementations: Vec::new(),
            }
        )
    }
}

impl Model {
    pub fn all_imports(&self) -> Vec<Import> {
        let mut field_items = self.fields.iter()
            .flat_map(|f| f.imports.clone())
            .collect::<Vec<_>>();

        let mut self_items = self.imports.clone();
        field_items.append(&mut self_items);

        field_items
    }
}
