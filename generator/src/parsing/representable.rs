use sw4rm_rs::shared::{SchemaType, SchemaTypeContainer};

use super::{error::GenerationError, file::FileInputParams, model::Model};


/// A representable item
#[derive(Debug, Clone)]
pub enum Representable {
    /// A model (a.k.a struct or enum)
    Model(Model),
}

impl TryInto<Vec<syn::Item>> for Representable {
    type Error = GenerationError;

    fn try_into(self) -> Result<Vec<syn::Item>, Self::Error> {
        match self {
            Self::Model(m) => m.try_into() 
        }
    }
}

impl TryFrom<FileInputParams> for Representable {
    type Error = GenerationError;

    fn try_from(value: FileInputParams) -> Result<Self, Self::Error> {
        let item_type = value.schema_item.schema_type.as_ref().map(|t| match t {
            SchemaTypeContainer::SingleType(t) => t,
            // TODO: bad
            SchemaTypeContainer::MultiType(ts) => ts.first().unwrap(),
        });

        let is_enum = !value.schema_item.enum_values.is_empty();
        let is_multitype = !value.schema_item.all_of.is_empty() || !value.schema_item.one_of.is_empty();

        // Will need to find a way to handle enums and allOf/oneOf/etc here
        match item_type {
            Some(SchemaType::Object) => return Ok(Self::Model(value.clone().try_into()?)),

            /// will need to find a way to handle this
            Some(SchemaType::Array) => debug!("Is array: {}", &value.definition_key),

            // will need to find a way to handle this
            _ if is_enum => debug!("Enum type: {item_type:?}"),

            // will need to find a way to handle this
            _ if is_multitype => debug!("Is all_of or one_of: {}", &value.definition_key),

            // will need to find a way to handle this
            None => warn!("Empty type! {}", &value.definition_key),

            // seems like sometimes there are just types? need to handle this better
            _ => error!("Unknown type for {}: {item_type:?}", &value.definition_key),
        };

        Err(GenerationError::Incomplete)
    }
}
