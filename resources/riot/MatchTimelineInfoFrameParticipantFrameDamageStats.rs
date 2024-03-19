#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MatchTimelineInfoFrameParticipantFrameDamageStats {
    pub magic_damage_done: !,
    pub total_damage_taken: !,
    pub true_damage_done_to_champions: !,
    pub true_damage_taken: !,
    pub total_damage_done_to_champions: !,
    pub magic_damage_taken: !,
    pub physical_damage_done: !,
    pub physical_damage_taken: !,
    pub physical_damage_done_to_champions: !,
    pub total_damage_done: !,
    pub magic_damage_done_to_champions: !,
    pub true_damage_done: !,
}
