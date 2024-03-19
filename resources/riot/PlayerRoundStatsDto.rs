#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PlayerRoundStatsDto {
    pub score: !,
    pub puuid: !,
}
