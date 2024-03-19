#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupsVtwoGroupFeatures {
    pub invite_permission_override: !,
    pub update_culture_permission_override: !,
    pub update_banner_permission_override: !,
    pub capabilities: !,
    pub join_level: !,
    pub maximum_members: !,
    pub maximum_memberships_of_group_type: !,
    pub host_guided_game_permission_override: !,
}
