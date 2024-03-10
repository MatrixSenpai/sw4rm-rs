use super::model::Model;

/// A full file
#[derive(Debug, Clone)]
pub struct File {
    /// The file name
    pub file_name: String,
    /// The file contents.
    pub contents: Vec<Representable>,
}

/// A representable item
#[derive(Debug, Clone)]
pub enum Representable {
    /// A model (a.k.a struct or enum)
    Model(Model),
}
