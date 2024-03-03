use std::collections::HashMap;
use convert_case::*;
use proc_macro2::Ident;
use syn::{
    File, Item, ItemUse, ItemMod, Token, UseName, UsePath, UseTree, Visibility,
    __private::Span,
};

use super::{
    GenerationError,
    item::parse_item,
};
use sw4rm_rs::{
    Spec,
    shared::Schema
};

/// Parses a list of files from a set of schemas. These items must have `resolve` called on them
pub fn parse_files(
    spec: &Spec,
    file_items: HashMap<&String, Schema>,
) -> Result<HashMap<String, File>, GenerationError> {
    let mut converted_items: HashMap<String, File> = file_items.into_iter()
        .map(|(file_name, schema)| (
            file_name.clone(), parse_file(spec, &file_name, schema).unwrap()
        ))
        .collect();

    let filenames = converted_items.keys()
        .map(|k| k.clone())
        .collect();

    converted_items.insert("mod".to_string(), parse_mod_file(filenames));

    Ok(converted_items)
}

// TODO: this should be more intelligent and only generate `use` for items that are actually required
/// Parse a single file from an OpenAPI specification and returns that file. Will also generate any
/// required `use` statements at the top of the file
pub fn parse_file(
    spec: &Spec,
    file_name: &String,
    schema: Schema,
) -> Result<File, GenerationError> {
    let mut items = generate_use_items(
        vec![vec!["std".to_string(), "collections".to_string(), "HashMap".to_string()]]
    );
    items.push(
        parse_item(spec, file_name, schema)?
    );

    Ok(
        File {
            items,
            attrs: Vec::default(),
            shebang: None,
        }
    )
}

pub fn parse_mod_file(filenames: Vec<String>) -> File {
    let items = filenames.iter()
        .map(|filename| Ident::new(
            filename.to_case(Case::Snake).as_str(), Span::call_site()
        ))
        .map(|item| ItemMod {
            attrs: Vec::new(),
            vis: Visibility::Public(Token![pub](Span::call_site())),
            unsafety: None,
            mod_token: Token![mod](Span::call_site()),
            ident: item,
            content: None,
            semi: None,
        })
        .map(Item::Mod)
        .collect();

    File {
        shebang: None,
        attrs: Vec::new(),
        items
    }
}

/// Takes a 2D array of strings and returns a 1D array of items. The array of strings will end up
/// as a tree path for `use`.
fn generate_use_items(items: Vec<Vec<String>>) -> Vec<Item> {
    items.into_iter()
        .map(generate_use_item)
        .collect()
}

/// Takes an array of strings and returns an item. Calls the `recursive_item_tree` to start off
/// generating a path to use as a `use` statement
fn generate_use_item(item: Vec<String>) -> Item {
    let item = ItemUse {
        attrs: Vec::default(),
        vis: Visibility::Inherited,
        use_token: Token![use](Span::call_site()),
        leading_colon: None,
        semi_token: Token![;](Span::call_site()),
        tree: recursive_item_tree(item),
    };

    Item::Use(item)
}

/// Recursively creates a tree to turn into a `use` statement. Removes each item from the array as
/// it moves through, starting with the first element. When there is a single element remaining,
/// returns that as an identifier
fn recursive_item_tree(path: Vec<String>) -> UseTree {
    let mut path = path;

    if path.len() > 1 {
        let item = path.remove(0);

        let tree_path = UsePath {
            ident: Ident::new(item.as_str(), Span::call_site()),
            colon2_token: Token![::](Span::call_site()),
            tree: Box::new(recursive_item_tree(path))
        };

        UseTree::Path(tree_path)
    } else {
        let item = path.remove(0);

        let tree_path = UseName {
            ident: Ident::new(item.as_str(), Span::call_site())
        };

        UseTree::Name(tree_path)
    }
}