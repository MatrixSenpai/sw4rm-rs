use crate::Error;
use crate::models::ResolveError;

#[derive(Debug)]
pub enum GenerationError {
    CrateError(Error),
    ResolveError(ResolveError),
    IoError(std::io::Error),
}

impl From<Error> for GenerationError {
    fn from(value: Error) -> Self {
        Self::CrateError(value)
    }
}

impl From<ResolveError> for GenerationError {
    fn from(value: ResolveError) -> Self {
        Self::ResolveError(value)
    }
}

impl From<std::io::Error> for GenerationError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}