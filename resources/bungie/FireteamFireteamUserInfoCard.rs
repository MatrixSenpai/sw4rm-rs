#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FireteamFireteamUserInfoCard {
    pub is_public: !,
    pub fireteam_membership_type: !,
    pub cross_save_override: !,
    pub membership_type: !,
    pub supplemental_display_name: !,
    pub membership_id: !,
    pub display_name: !,
    pub bungie_global_display_name_code: !,
    pub bungie_global_display_name: !,
    pub fireteam_display_name: !,
    pub icon_path: !,
}
