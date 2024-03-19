#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TournamentGamesVfive {
    pub game_name: !,
    pub game_type: !,
    pub game_mode: !,
    pub meta_data: !,
    pub game_id: !,
    pub short_code: !,
    pub region: !,
    pub game_map: !,
}
