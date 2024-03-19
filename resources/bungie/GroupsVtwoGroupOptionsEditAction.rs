#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupsVtwoGroupOptionsEditAction {
    pub host_guided_game_permission_override: !,
    pub update_banner_permission_override: !,
    pub join_level: !,
    pub invite_permission_override: !,
    pub update_culture_permission_override: !,
}
