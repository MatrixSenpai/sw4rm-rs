use lazy_static::lazy_static;
use sanitizer::prelude::*;
use regex::{Captures, Regex};
use convert_case::{Case, Casing};
use super::error::GenerationError;

lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug, Clone)]
pub struct Identifier(String, bool);
impl Identifier {
    pub fn new(identifier: String) -> Self {
        let num_sanitized = NUM_REGEX.replace_all(identifier.as_str(), |number: &Captures| {
            rust_numerals::number_to_cardinal(
                number[0].parse::<i64>().unwrap()
            )
        }).to_string();

        let mut sn_instance = StringSanitizer::from(num_sanitized); 
        sn_instance.alphanumeric();
        let an_only_sanitized = sn_instance.get();

        let did_change = identifier.ne(&an_only_sanitized);
        
        Self(an_only_sanitized, did_change)
    }

    pub fn lowercased(&self, is_using_lower_transform: bool) -> (String, bool) {
        self.change_rep(Case::Lower, is_using_lower_transform)
    }

    pub fn field_rep(&self, is_using_snake_transform: bool) -> (String, bool) {
        self.change_rep(Case::Snake, is_using_snake_transform)
    }

    pub fn enum_rep(&self, is_using_camel_transform: bool) -> (String, bool) {
        self.change_rep(Case::Camel, is_using_camel_transform)
    }

    pub fn change_rep(&self, rep: Case, is_using_transform: bool) -> (String, bool) {
        let cased = self.0.to_case(rep);
        let did_change = self.0.ne(&cased);

        (
            cased,
            self.1 || (did_change && is_using_transform)
        )
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
impl TryFrom<Option<String>> for Identifier {
    type Error = GenerationError;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        value.map(Self::new).ok_or(GenerationError::UnknownIdentifier)
    }
}
impl<T> TryFrom<Result<String, T>> for Identifier {
    type Error = T;

    fn try_from(value: Result<String, T>) -> Result<Self, Self::Error> {
        value.map(Self::new)
    }
}
