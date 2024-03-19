#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ForumPostResponse {
    pub locale: !,
    pub is_pinned: !,
    pub last_reply_timestamp: !,
    pub user_has_muted_post: !,
    pub thumbnail: !,
    pub is_active: !,
    pub is_announcement: !,
    pub latest_reply_post_id: !,
    pub url_media_type: !,
    pub popularity: !,
    pub user_has_rated: !,
    pub latest_reply_author_id: !,
    pub user_rating: !,
}
