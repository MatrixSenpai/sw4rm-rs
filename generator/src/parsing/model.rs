use super::{
    import::Import,
    attribute::Attribute,
    field::Field,
};

/// A struct representation
#[derive(Debug, Clone)]
pub struct Model {
    /// All imports that this model uses. Can be from another crate or part of this one
    pub imports: Vec<Import>,
    /// The documentation string that gets added to the model
    pub documentation: String,
    /// All attributes to be added to this model. E.g. derive, serde(rename = "foo"), serde(default)
    pub attributes: Vec<Attribute>,
    /// The name of the modifier. Should be `Ident` ready
    pub model_name: String,
    /// Model fields
    pub fields: Vec<Field>,
    // TODO: this will be mildly more complex
    /// All manual implementations for the model. E.g. impl Deserialize for Foo
    pub implementations: Vec<()>,
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
