
#[derive(Debug, Clone)]
pub enum GenerationError {
    // TODO: remove
    Incomplete,
    MissingType,
    UnknownIdentifier,
}
