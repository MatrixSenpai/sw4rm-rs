#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchResultOfGroupMembership {
    pub has_more: !,
    pub use_total_results: !,
    pub total_results: !,
    pub replacement_continuation_token: !,
}
