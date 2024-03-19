#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LeagueItemDto {
    pub fresh_blood: !,
    pub veteran: !,
    pub inactive: !,
    pub summoner_id: !,
    pub losses: !,
    pub league_points: !,
    pub hot_streak: !,
    pub wins: !,
    pub summoner_name: !,
    pub rank: !,
}
