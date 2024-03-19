#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MatchlistEntryDto {
    pub queue_id: !,
    pub match_id: !,
    pub game_start_time_millis: !,
}
