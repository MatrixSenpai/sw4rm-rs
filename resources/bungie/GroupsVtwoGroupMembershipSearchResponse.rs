#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupsVtwoGroupMembershipSearchResponse {
    pub total_results: !,
    pub use_total_results: !,
    pub has_more: !,
    pub replacement_continuation_token: !,
}
