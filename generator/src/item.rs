use proc_macro2::Ident;
use syn::{
    Fields, FieldsNamed, Generics, Item, ItemStruct, Token, Visibility,
    token::Brace,
    __private::Span,
};

use super::{
    GenerationError,
    attribute::parse_struct_attrs,
    field::parse_struct_fields,
};
use sw4rm_rs::{
    Spec,
    shared::Schema,
};

// TODO: should generate a struct or an enum depending on values
/// Creates a full item from a schema
pub fn parse_item(
    spec: &Spec,
    file_name: &String,
    schema: Schema,
) -> Result<Item, GenerationError> {
    let ident = Ident::new(
        schema.title.clone().unwrap_or(file_name.clone()).as_str(),
        Span::call_site(),
    );
    let attrs = parse_struct_attrs(schema.description.clone());
    let fields = FieldsNamed {
        named: parse_struct_fields(spec, &schema)?,
        brace_token: Brace::default(),
    };

    Ok(
        Item::Struct(ItemStruct {
            ident, attrs,
            fields: Fields::Named(fields),
            vis: Visibility::Public(Token![pub](Span::call_site())),
            semi_token: None,
            struct_token: Token![struct](Span::call_site()),
            generics: Generics::default(),
        })
    )
}
