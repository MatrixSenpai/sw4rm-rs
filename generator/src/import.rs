
/// An import that an item will use
#[derive(Debug, Clone)]
pub struct Import {
    /// The path items to this import. The `std::collections` in `std::collections::HashMap`
    pub path: Vec<String>,
    /// The item or items to import. The `HashMap` in `std::collections::HashMap`
    pub item: String,
}
