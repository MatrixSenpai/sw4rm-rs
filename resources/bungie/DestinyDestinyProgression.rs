#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDestinyProgression {
    pub level_cap: !,
    pub weekly_progress: !,
    pub level: !,
    pub progression_hash: !,
    pub daily_limit: !,
    pub weekly_limit: !,
    pub current_reset_count: !,
    pub current_progress: !,
    pub next_level_at: !,
    pub step_index: !,
    pub progress_to_next_level: !,
    pub daily_progress: !,
}
