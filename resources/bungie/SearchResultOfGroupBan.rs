#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchResultOfGroupBan {
    pub use_total_results: !,
    pub has_more: !,
    pub total_results: !,
    pub replacement_continuation_token: !,
}
