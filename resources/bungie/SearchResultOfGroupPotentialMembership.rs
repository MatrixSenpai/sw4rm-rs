#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchResultOfGroupPotentialMembership {
    pub has_more: !,
    pub total_results: !,
    pub replacement_continuation_token: !,
    pub use_total_results: !,
}
