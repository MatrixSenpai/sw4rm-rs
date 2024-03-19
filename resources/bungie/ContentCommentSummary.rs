#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContentCommentSummary {
    pub topic_id: !,
    pub comment_count: !,
}
