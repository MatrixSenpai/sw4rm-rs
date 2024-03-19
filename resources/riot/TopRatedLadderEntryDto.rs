#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TopRatedLadderEntryDto {
    pub wins: !,
    pub rated_tier: !,
    pub summoner_id: !,
    pub summoner_name: !,
    pub previous_update_ladder_position: !,
    pub rated_rating: !,
}
