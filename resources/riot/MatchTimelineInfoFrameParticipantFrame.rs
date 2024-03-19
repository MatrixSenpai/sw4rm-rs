#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MatchTimelineInfoFrameParticipantFrame {
    pub level: !,
    pub participant_id: !,
    pub time_enemy_spent_controlled: !,
    pub xp: !,
    pub gold_per_second: !,
    pub minions_killed: !,
    pub jungle_minions_killed: !,
    pub total_gold: !,
    pub current_gold: !,
}
