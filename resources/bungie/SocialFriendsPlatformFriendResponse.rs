#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SocialFriendsPlatformFriendResponse {
    pub current_page: !,
    pub has_more: !,
    pub items_per_page: !,
}
