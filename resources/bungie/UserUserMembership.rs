#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserUserMembership {
    pub bungie_global_display_name: !,
    pub bungie_global_display_name_code: !,
    pub display_name: !,
    pub membership_type: !,
    pub membership_id: !,
}
