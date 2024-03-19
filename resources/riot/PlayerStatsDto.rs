#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PlayerStatsDto {
    pub kills: !,
    pub rounds_played: !,
    pub deaths: !,
    pub assists: !,
    pub score: !,
    pub playtime_millis: !,
}
