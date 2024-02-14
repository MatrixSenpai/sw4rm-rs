use crate::{RefType, ResolveError};

pub trait Resolvable {
    fn resolve(root: &impl ResolveRoot, path: &str) -> Result<Self, ResolveError> where Self: Sized;
}

pub trait ResolveRoot {
    fn resolve<T: Resolvable>(&self, item_type: RefType, path: &str) -> Result<T, ResolveError>;
}
