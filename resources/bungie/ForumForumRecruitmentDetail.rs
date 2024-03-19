#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ForumForumRecruitmentDetail {
    pub player_slots_total: !,
    pub tone: !,
    pub conversation_id: !,
    pub intensity: !,
    pub microphone_required: !,
    pub approved: !,
    pub topic_id: !,
    pub player_slots_remaining: !,
}
