#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupsVtwoGroupMember {
    pub join_date: !,
    pub is_online: !,
    pub last_online_status_change: !,
    pub member_type: !,
    pub group_id: !,
}
