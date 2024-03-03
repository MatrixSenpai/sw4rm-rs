use convert_case::{Case, Casing};
use proc_macro2::Ident;
use regex::Captures;
use syn::__private::Span;

use super::constants::*;

/// Parses a field identifier, searching for illegal identifiers
/// Returns a tuple of the new identifier, plus whether it needs a serde rename attr
pub fn parse_field_ident(
    item_name: &String,
) -> (Ident, bool) {
    let cased = item_name.clone().to_case(Case::Snake);
    let number_checked = NUMBER_REGEX.replace(&cased.as_str(), |cap: &Captures| {
        let number = cap["number"].parse::<i64>().unwrap();
        rust_numerals::number_to_cardinal(number)
    });
    let type_checked = TYPE_REGEX.replace(&number_checked, "kind");
    let match_checked = MATCH_REGEX.replace(&type_checked, "match_item");

    let ident = Ident::new(&match_checked, Span::call_site());
    let needs_rename = cased.ne(&match_checked) || HAS_NUMBER_REGEX.is_match(item_name);

    (ident, needs_rename)
}
