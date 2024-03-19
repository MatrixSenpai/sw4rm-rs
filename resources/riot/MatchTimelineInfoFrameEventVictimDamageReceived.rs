#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MatchTimelineInfoFrameEventVictimDamageReceived {
    pub magic_damage: !,
    pub spell_name: !,
    pub true_damage: !,
    pub spell_slot: !,
    pub participant_id: !,
    pub name: !,
    pub type: !,
    pub physical_damage: !,
    pub basic: !,
}
