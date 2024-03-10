
/// An attribute to be applied. E.g. `#[derive(Debug)]`, `#[command]`
#[derive(Debug, Clone)]
pub enum Attribute {
    /// A single attribute. E.g. `#[command]`
    Single(String),
    /// A parenthesized attribute. E.g. `#[derive(Debug)]`
    Paren(ParenAttribute),
}

/// A parenthesized attribute. E.g. `#[derive(Debug)]`
#[derive(Debug, Clone)]
pub struct ParenAttribute {
    /// The name outside parenthesis. The `derive` in `#[derive(Debug)]`
    pub name: String,
    /// The items inside the parenthesis. The `Debug` in `#[derive(Debug)]`
    pub items: Vec<ParenAttributeItem>,
}

/// A parenthesized attribute. E.g. `Debug`, `rename_all = "foo"`
#[derive(Debug, Clone)]
pub enum ParenAttributeItem {
    /// A single item. E.g. `Debug`
    Single(String),
    /// An assigned value. E.g. `rename_all = "foo"`
    AssignValue(String, String),
}
