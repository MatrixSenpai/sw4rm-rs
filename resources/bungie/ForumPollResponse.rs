#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ForumPollResponse {
    pub topic_id: !,
    pub total_votes: !,
}
