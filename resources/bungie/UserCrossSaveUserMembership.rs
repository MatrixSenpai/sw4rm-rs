#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserCrossSaveUserMembership {
    pub membership_type: !,
    pub membership_id: !,
    pub is_public: !,
    pub bungie_global_display_name: !,
    pub cross_save_override: !,
    pub display_name: !,
    pub bungie_global_display_name_code: !,
}
