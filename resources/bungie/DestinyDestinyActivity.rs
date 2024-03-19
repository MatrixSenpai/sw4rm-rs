#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDestinyActivity {
    pub recommended_light: !,
    pub activity_hash: !,
    pub difficulty_tier: !,
    pub loadout_requirement_index: !,
    pub display_level: !,
    pub can_lead: !,
    pub can_join: !,
    pub is_visible: !,
    pub is_new: !,
    pub is_completed: !,
}
