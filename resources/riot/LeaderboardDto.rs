#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LeaderboardDto {
    pub query: !,
    pub shard: !,
    pub act_id: !,
    pub top_tier_rr_threshold: !,
    pub start_index: !,
    pub immortal_starting_index: !,
    pub immortal_starting_page: !,
    pub total_players: !,
}
