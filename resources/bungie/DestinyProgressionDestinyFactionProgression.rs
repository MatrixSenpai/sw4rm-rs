#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyProgressionDestinyFactionProgression {
    pub level: !,
    pub faction_vendor_index: !,
    pub daily_limit: !,
    pub current_reset_count: !,
    pub step_index: !,
    pub progression_hash: !,
    pub daily_progress: !,
    pub weekly_progress: !,
    pub level_cap: !,
    pub progress_to_next_level: !,
    pub current_progress: !,
    pub next_level_at: !,
    pub faction_hash: !,
    pub weekly_limit: !,
}
