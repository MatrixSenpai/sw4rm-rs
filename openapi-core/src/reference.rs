use crate::Spec;
use anyhow::{Context, bail};
use regex::Regex;
use serde::de::{DeserializeOwned, Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Formatter;

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(untagged)]
pub enum RefOr<T> {
    Object(T),
    Reference {
        #[serde(rename = "$ref")]
        location: Reference,
        summary: Option<String>,
        description: Option<String>,
    },
}

impl<T: Resolvable> RefOr<T> {
    fn resolve<'a>(&'a self, spec: &'a Spec) -> anyhow::Result<&'a T> {
        match self {
            Self::Object(v) => Ok(v),
            Self::Reference { location, .. } => {
                T::resolve(spec, location)
            }
        }
    }
}

pub trait Resolvable: Sized + Clone {
    fn resolve<'a>(spec: &'a Spec, reference: &Reference) -> anyhow::Result<&'a Self>;
}

#[derive(Clone, Debug, Serialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Reference {
    pub source: Option<String>,
    pub location: ReferenceLocation,
    pub kind: Option<ReferenceKind>,
    pub name: Option<String>,
}

impl Reference {
    pub fn resolve<T: Resolvable + DeserializeOwned>(
        &self,
        spec: &Spec,
        dir_search_base: Option<String>,
    ) -> anyhow::Result<&T> {
        if self.source.is_some() {
            self.resolve_external(dir_search_base)
        } else {
            T::resolve(spec, self)
        }
    }

    pub fn resolve_external<T: DeserializeOwned>(
        &self,
        dir_search_base: Option<String>,
    ) -> anyhow::Result<&T> {
        let source = self
            .source
            .clone()
            .context("Expected an actual external resolution to take place!")?;
        let location = match dir_search_base {
            Some(v) => format!("{v}/{source}"),
            None => format!("./{source}"),
        };
        let external = std::fs::read_to_string(location).context("Cannot read external file!")?;

        let is_json = source.contains(".json");
        let is_yaml = source.contains(".yml") || source.contains(".yaml");
        let de: HashMap<String, Value> = match (is_json, is_yaml) {
            (true, _) => serde_json::from_str(&external).context("json error")?,
            (_, true) => serde_yaml::from_str(&external).context("yaml error")?,
            _ => bail!("Externally referenced source file is neither json nor yaml, cannot decode"),
        };

        let key = self.location.to_string();
        let value = de
            .get(&key)
            .context("Externally referenced source file does not contain required key!")?
            .clone();
        let decoded = serde_json::from_value(value)
            .context("Externally referenced item is not the expected type!")?;
        Ok(decoded)
    }
}

impl TryFrom<String> for Reference {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let ext_file = Regex::new(r#"^(?<fn>.*)\.(?<ext>json|ya?ml)#?"#).context("bad regex")?;
        let internal = Regex::new(r"^([^#]*)#/(?<location>[^/]+)/*(?<kind>[^/]*)/*(?<name>\S*)$")
            .context("bad regex")?;

        let path_matches = internal.captures(&value).context("Malformed path!")?;

        let source = ext_file.captures(&value).and_then(|c| {
            let file_path = c["fn"].to_string();
            let ext = c["ext"].to_string();
            Some(format!("{file_path}.{ext}"))
        });

        let location = path_matches["location"].to_string().into();
        let kind = path_matches.name("kind").and_then(|v| {
            let val = v.as_str();
            if val.is_empty() {
                None
            } else {
                Some(val.to_string().into())
            }
        });
        let name = path_matches.name("name").and_then(|v| {
            let val = v.as_str();
            if val.is_empty() {
                None
            } else {
                Some(val.to_string())
            }
        });

        Ok(Self {
            source,
            location,
            kind,
            name,
        })
    }
}

impl<'de> Deserialize<'de> for Reference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(ReferenceVisitor)
    }
}

struct ReferenceVisitor;
impl<'de> Visitor<'de> for ReferenceVisitor {
    type Value = Reference;
    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Unable to decode reference!")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        v.to_string()
            .try_into()
            .map_err(|e| E::custom(format!("{e:?}")))
    }
    fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
        v.try_into().map_err(|e| E::custom(format!("{e:?}")))
    }
}

#[derive(Clone, Debug, Hash, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ReferenceLocation {
    Servers,
    Paths,
    Webhooks,
    Components,
    Security,
    Tags,
    Unknown(String),
}
impl From<String> for ReferenceLocation {
    fn from(value: String) -> Self {
        match value.as_str() {
            "servers" => Self::Servers,
            "paths" => Self::Paths,
            "webhooks" => Self::Webhooks,
            "components" => Self::Components,
            "security" => Self::Security,
            "tags" => Self::Tags,
            _ => Self::Unknown(value),
        }
    }
}
impl std::fmt::Display for ReferenceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::Servers => "servers".to_string(),
            Self::Paths => "paths".to_string(),
            Self::Webhooks => "webhooks".to_string(),
            Self::Components => "components".to_string(),
            Self::Security => "security".to_string(),
            Self::Tags => "tags".to_string(),
            Self::Unknown(v) => v.clone(),
        };
        write!(f, "{v}")
    }
}

#[derive(Clone, Debug, Hash, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ReferenceKind {
    Schemas,
    Responses,
    Parameters,
    Examples,
    RequestBodies,
    Headers,
    SecuritySchemes,
    Links,
    Callbacks,
    PathItems,
    Unknown(String),
}
impl From<String> for ReferenceKind {
    fn from(value: String) -> Self {
        match value.as_str() {
            "schemas" => Self::Schemas,
            "responses" => Self::Responses,
            "parameters" => Self::Parameters,
            "examples" => Self::Examples,
            "requestBodies" => Self::RequestBodies,
            "headers" => Self::Headers,
            "securitySchemes" => Self::SecuritySchemes,
            "links" => Self::Links,
            "callbacks" => Self::Callbacks,
            "pathItems" => Self::PathItems,
            _ => Self::Unknown(value),
        }
    }
}
