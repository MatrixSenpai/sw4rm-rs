#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchResultOfDestinyEntitySearchResultItem {
    pub use_total_results: !,
    pub has_more: !,
    pub replacement_continuation_token: !,
    pub total_results: !,
}
