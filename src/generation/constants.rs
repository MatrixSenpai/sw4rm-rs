use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref HAS_NUMBER_REGEX: Regex = Regex::new(
        r"\d+",
    ).unwrap();
    pub static ref NUMBER_REGEX: Regex = Regex::new(
        r"^(?<number>\d+)",
    ).unwrap();
    pub static ref TYPE_REGEX: Regex = Regex::new(
        r"^(?<item_type>type)$",
    ).unwrap();
    pub static ref MATCH_REGEX: Regex = Regex::new(
        r"^(?<item_match>match)$",
    ).unwrap();
}
