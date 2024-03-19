use std::sync::Arc;
use sw4rm_rs::{
    *, shared::*, openapi_v2::*, openapi_v3_0::*,
};

use super::{error::GenerationError, identifier::Identifier, model::Model, representable::Representable};

/// A full file
#[derive(Debug, Clone)]
pub struct File {
    /// The file name
    pub file_name: Identifier,
    /// The file contents.
    pub contents: Representable,
}

impl File {
    fn new(file_name: String, contents: Representable) -> Self {
        Self { file_name: file_name.into(), contents }
    }
}

impl TryInto<syn::File> for File {
    type Error = GenerationError;

    fn try_into(self) -> Result<syn::File, Self::Error> {
        Ok(
            syn::File {
                shebang: None,
                attrs: Vec::new(),
                items: self.contents.try_into()?,
            }
        )
    }
}

// TODO: rename
/// The params to use for transforming
#[derive(Debug, Clone)]
pub struct FileInputParams {
    pub definition_key: String,
    pub schema_item: Schema,
    pub spec: Arc<Spec>,
}

/// Attempt to decode from a tuple of `(String, Schema)`. `String` should be the definition key
impl TryFrom<FileInputParams> for File {
    // TODO: change
    type Error = GenerationError;

    fn try_from(value: FileInputParams) -> Result<Self, Self::Error> {
        // TODO: should modify file name to check for special chars and such. Should always prefer
        // the schema title over the definition key if available.
        let file_name = value.clone().schema_item.title.unwrap_or(value.clone().definition_key);

        Ok(Self::new(file_name, value.try_into()?))
    }
}
