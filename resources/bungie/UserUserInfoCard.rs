#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserUserInfoCard {
    pub display_name: !,
    pub icon_path: !,
    pub membership_type: !,
    pub membership_id: !,
    pub is_public: !,
    pub bungie_global_display_name: !,
    pub supplemental_display_name: !,
    pub bungie_global_display_name_code: !,
    pub cross_save_override: !,
}
