#[macro_use]
extern crate resolve_macro;

pub extern crate resolve_core;

pub mod models;
pub mod error;

use std::{
    fs::File, io::Read, path::Path,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use resolve_core::ResolveRoot;

use crate::error::Error;

pub fn from_path<T, P>(path: P) -> Result<T, Error>
where
    T: ResolveRoot + DeserializeOwned,
    P: AsRef<Path>,
{
    from_reader(File::open(path)?)
}

pub fn from_reader<T, R>(reader: R) -> Result<T, Error>
where
    T: ResolveRoot + DeserializeOwned,
    R: Read,
{
    Ok(serde_yaml::from_reader(reader)?)
}

pub fn to_json<T>(spec: T) -> Result<String, Error>
where
    T: ResolveRoot + Serialize
{
    Ok(serde_json::to_string(spec)?)
}

pub fn to_yaml<T>(spec: T) -> Result<String, Error>
where
    T: ResolveRoot + Serialize
{
    Ok(serde_yaml::to_string(spec)?)
}