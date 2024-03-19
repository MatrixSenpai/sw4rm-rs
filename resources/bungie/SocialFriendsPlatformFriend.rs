#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SocialFriendsPlatformFriend {
    pub bungie_global_display_name_code: !,
    pub friend_platform: !,
    pub platform_display_name: !,
    pub destiny_membership_id: !,
    pub destiny_membership_type: !,
    pub bungie_net_membership_id: !,
    pub bungie_global_display_name: !,
}
