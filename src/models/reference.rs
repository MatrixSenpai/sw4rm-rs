use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use regex::Regex;
use crate::models::spec::Spec;

lazy_static! {
    static ref PATH_EXPRESSION: Regex = Regex::new(r"^(?<source>[^#]*)#\/(?<location>[^/]+)\/(?<kind>[^/]+)\/*(?<name>\S*)$").unwrap();
}

pub trait Resolvable: Clone + Sized {
    fn resolve(spec: &Spec, path: &String) -> Result<Self, ResolveError>;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum RefOr<T> where T: Resolvable {
    Reference {
        #[serde(rename = "$ref")]
        reference_path: String,
    },
    Item(T),
}
impl<T> RefOr<T> where T: Resolvable {
    pub fn resolve(&self, spec: &Spec) -> Result<T, ResolveError> {
        match self {
            Self::Item(item) => Ok(item.clone()),
            Self::Reference { reference_path } => T::resolve(spec, reference_path)
        }
    }
}
impl<T> Resolvable for Box<T> where T: Resolvable {
    fn resolve(spec: &Spec, path: &String) -> Result<Self, ResolveError> {
        Ok(Box::new(T::resolve(spec, path)?))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ReferenceType {
    Callbacks,
    Definitions,
    Example,
    Link,
    Parameter,
    PathItem,
    RequestBody,
    Response,
    Schema,
    SecurityScheme,
}
impl TryFrom<String> for ReferenceType {
    type Error = ResolveError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "callback" => Ok(Self::Callbacks),
            "definitions" => Ok(Self::Definitions),
            "example" => Ok(Self::Example),
            "link" => Ok(Self::Link),
            "parameter" | "parameters" => Ok(Self::Parameter),
            "path_item" => Ok(Self::PathItem),
            "request_body" => Ok(Self::RequestBody),
            "response" => Ok(Self::Response),
            "schemas" => Ok(Self::Schema),
            "security_scheme" => Ok(Self::SecurityScheme),

            #[allow(unreachable_patterns)]
            _ => Err(Self::Error::UnknownType)
        }
    }
}

#[derive(Debug)]
pub enum ResolveError {
    UnknownType,
    MissingPathError,
    MissingLocationError,
    MissingKindError,
    MissingMatchesError,
    UnknownPathError(String),
}

#[derive(Clone, Debug)]
pub struct Reference {
    pub source: Option<String>,
    pub location: String,
    pub kind: ReferenceType,
    pub name: String,
}

impl TryFrom<String> for Reference {
    type Error = ResolveError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let matches = PATH_EXPRESSION.captures(value.as_str())
            .ok_or(Self::Error::MissingMatchesError)?;

        let source = matches.name("source")
            .map(|v| v.as_str().to_string().clone());

        let location = matches.name("location")
            .ok_or(Self::Error::MissingLocationError)?
            .as_str().to_string().clone();

        let kind_item = matches.name("kind")
            .ok_or(Self::Error::MissingKindError)?
            .as_str().to_string().clone();
        let kind: ReferenceType = match kind_item.clone().try_into() {
            Ok(v) => v,
            Err(_) => location.clone().try_into()?,
        };

        let name_item = matches.name("name")
            .map(|v| v.as_str().to_string().clone());
        let name = match name_item {
            Some(v) if !v.is_empty() => v,
            _ => kind_item.clone()
        };

        Ok(Self {
            source, location, kind, name
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Reference;

    #[test]
    fn test_regex_captures_swagger_yaml() {
        let hay = "#/parameters/nested_response";
        let path = Reference::try_from(hay.to_string()).unwrap();
        println!("{path:?}")
    }
    #[test]
    fn test_regex_captures_swagger_json() {
        let hay = "#/definitions/account-v1.AccountDto";
        let path = Reference::try_from(hay.to_string()).unwrap();
        println!("{path:?}")
    }
    #[test]
    fn test_regex_captures_openapi_yaml() {
        let hay = "#/components/schemas/Pets";
        let path = Reference::try_from(hay.to_string()).unwrap();
        println!("{path:?}")
    }
    #[test]
    fn test_regex_captures_openapi_json() {
        let hay = "#/components/schemas/account-v1.AccountDto";
        let path = Reference::try_from(hay.to_string()).unwrap();
        println!("{path:?}")
    }
}