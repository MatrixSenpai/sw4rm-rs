#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupsVtwoGetGroupsForMemberResponse {
    pub replacement_continuation_token: !,
    pub use_total_results: !,
    pub has_more: !,
    pub total_results: !,
}
