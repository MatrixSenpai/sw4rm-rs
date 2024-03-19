#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InfoDto {
    pub platform_id: !,
    pub map_id: !,
    pub queue_id: !,
    pub game_end_timestamp: !,
    pub game_start_timestamp: !,
    pub game_version: !,
    pub game_name: !,
    pub game_mode: !,
    pub game_type: !,
    pub tournament_code: !,
    pub game_id: !,
    pub game_duration: !,
    pub game_creation: !,
}
