#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FeaturedGameInfo {
    pub game_type: !,
    pub platform_id: !,
    pub game_mode: !,
    pub game_length: !,
    pub map_id: !,
    pub game_id: !,
    pub game_queue_config_id: !,
}
