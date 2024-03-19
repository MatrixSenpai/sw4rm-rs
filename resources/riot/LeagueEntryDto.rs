#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LeagueEntryDto {
    pub summoner_id: !,
    pub league_points: !,
    pub rated_tier: !,
    pub rated_rating: !,
    pub tier: !,
    pub puuid: !,
    pub rank: !,
    pub summoner_name: !,
    pub hot_streak: !,
    pub inactive: !,
    pub league_id: !,
    pub queue_type: !,
    pub wins: !,
    pub losses: !,
    pub veteran: !,
    pub fresh_blood: !,
}
