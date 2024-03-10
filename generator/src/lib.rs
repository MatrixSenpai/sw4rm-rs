#![allow(dead_code, unused)]

mod file;
mod constants;
mod error;
mod item;
mod attribute;
mod field;
mod ident;
mod typed;
mod parsing;

use std::{
    collections::HashMap,
    path::Path,
};
use syn::File;

use sw4rm_rs::Spec;
use file::{
    parse_files,
    parse_mod_file,
};
use error::GenerationError;

pub enum ContainerItem {
    File(String, File),
    FileList(String, HashMap<String, File>),
}

pub struct GenerationOptions {

}

/// Takes a OpenAPI specification as input, and parses it out to a hashmap of files and file names
/// for writing. The OpenAPI spec MUST be parsed by this library, and the files returned are parsed
/// out as [syn::File](Syn File) types.
pub fn parse_spec(spec: Spec) -> Result<HashMap<String, File>, GenerationError> {
    let models = spec.schemas();
    let models_resolved = models.iter()
        .map(|(file_name, schema_item)| (file_name, schema_item.resolve(&spec).unwrap()))
        .collect();

    let files = parse_files(&spec, models_resolved)?;
    Ok(files)
}

/// A convenience function to parse a single file
pub fn parse_file<T: AsRef<Path>>(path: T) -> Result<HashMap<String, File>, GenerationError> {
    let spec = sw4rm_rs::from_path(path)?;
    parse_spec(spec)
}

/// A convenience function to parse a directory
pub fn parse_directory<T: AsRef<Path>>(path: T) -> Result<Vec<ContainerItem>, GenerationError> {
    let directory = std::fs::read_dir(path)?;

    let mut results = Vec::new();
    for file in directory.into_iter() {
        let file_path = file?.path();
        let result = parse_file(&file_path)?;

        results.push(
            ContainerItem::FileList(
                file_path.to_str().unwrap().to_string(),
                result
            )
        );
    }

    let dirnames = results.iter()
        .map(|i| {
            match i {
                ContainerItem::FileList(n, _) => n,
                ContainerItem::File(n, _) => n,
            }
        })
        .map(|k| k.clone())
        .collect();
    results.push(ContainerItem::File("mod".to_string(), parse_mod_file(dirnames)));

    Ok(results)
}

/// Writes a single file to the current directory as a rust file
pub fn write_file(name: &String, file: &File) {
    let formatted = prettyplease::unparse(file);
    std::fs::write(
        format!("./{}.rs", name),
        formatted
    ).unwrap();
}

/// Writes a list of files to the current directory, creating the directory and mod file inside
pub fn write_files(dirname: &String, file_list: &HashMap<String, File>) {
    match std::fs::read_dir(dirname) {
        Ok(_) => (),
        Err(_) => std::fs::create_dir_all(dirname).unwrap(),
    }

    for (filename, file) in file_list.iter() {
        write_file(
            &format!("{}/{}", dirname, filename),
            file
        )
    }
}

/// Writes a set of subdirectories, including mod files
pub fn write_directory(items: Vec<ContainerItem>) {
    for item in items.iter() {
        match item {
            ContainerItem::File(name, file) => write_file(name, file),
            ContainerItem::FileList(dirname, files) => write_files(dirname, files),
        }
    }
}

/// A convenience function to read and write from a single spec
pub fn read_parse_write_single_spec<T: AsRef<Path>>(path: T) -> Result<(), GenerationError> {
    let items = parse_file(path)?;
    write_files(&".".to_string(), &items);

    Ok(())
}

/// A convenience function to read and write from a full directory
pub fn read_parse_write_multi_spec<T: AsRef<Path>>(path: T) -> Result<(), GenerationError> {
    let items = parse_directory(path)?;
    write_directory(items);

    Ok(())
}