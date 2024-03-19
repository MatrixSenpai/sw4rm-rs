#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChampionMasteryDto {
    pub champion_points: !,
    pub champion_points_until_next_level: !,
    pub puuid: !,
    pub last_play_time: !,
    pub champion_points_since_last_level: !,
    pub chest_granted: !,
    pub champion_level: !,
    pub champion_id: !,
    pub summoner_id: !,
    pub tokens_earned: !,
}
