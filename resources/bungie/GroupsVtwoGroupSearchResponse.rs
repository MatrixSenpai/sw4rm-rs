#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupsVtwoGroupSearchResponse {
    pub use_total_results: !,
    pub replacement_continuation_token: !,
    pub total_results: !,
    pub has_more: !,
}
