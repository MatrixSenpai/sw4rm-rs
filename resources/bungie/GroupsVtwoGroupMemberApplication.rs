#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupsVtwoGroupMemberApplication {
    pub group_id: !,
    pub resolve_message: !,
    pub request_message: !,
    pub resolve_date: !,
    pub resolve_state: !,
    pub resolved_by_membership_id: !,
    pub creation_date: !,
}
