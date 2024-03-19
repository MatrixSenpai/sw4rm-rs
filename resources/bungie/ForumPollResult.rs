#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ForumPollResult {
    pub answer_slot: !,
    pub votes: !,
    pub last_vote_date: !,
    pub requesting_user_voted: !,
    pub answer_text: !,
}
