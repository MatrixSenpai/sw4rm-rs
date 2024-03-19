#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchResultOfGroupMemberApplication {
    pub use_total_results: !,
    pub total_results: !,
    pub has_more: !,
    pub replacement_continuation_token: !,
}
