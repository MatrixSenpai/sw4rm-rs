#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MatchInfoDto {
    pub custom_game_name: !,
    pub game_length_millis: !,
    pub provisioning_flow_id: !,
    pub queue_id: !,
    pub season_id: !,
    pub game_mode: !,
    pub is_completed: !,
    pub game_start_millis: !,
    pub match_id: !,
    pub is_ranked: !,
    pub map_id: !,
}
