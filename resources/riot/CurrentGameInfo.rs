#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CurrentGameInfo {
    pub game_id: !,
    pub game_length: !,
    pub map_id: !,
    pub platform_id: !,
    pub game_type: !,
    pub game_queue_config_id: !,
    pub game_start_time: !,
    pub game_mode: !,
}
