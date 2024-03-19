#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SocialFriendsBungieFriend {
    pub online_title: !,
    pub last_seen_as_bungie_membership_type: !,
    pub bungie_global_display_name: !,
    pub bungie_global_display_name_code: !,
    pub relationship: !,
    pub last_seen_as_membership_id: !,
    pub online_status: !,
}
