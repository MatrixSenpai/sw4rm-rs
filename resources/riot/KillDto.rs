#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct KillDto {
    pub killer: !,
    pub time_since_game_start_millis: !,
    pub victim: !,
    pub time_since_round_start_millis: !,
}
