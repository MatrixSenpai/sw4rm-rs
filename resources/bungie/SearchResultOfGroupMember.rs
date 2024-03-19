#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchResultOfGroupMember {
    pub has_more: !,
    pub replacement_continuation_token: !,
    pub total_results: !,
    pub use_total_results: !,
}
