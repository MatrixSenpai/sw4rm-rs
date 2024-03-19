#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChallengeConfigInfoDto {
    pub id: !,
    pub start_timestamp: !,
    pub tracking: !,
    pub leaderboard: !,
    pub end_timestamp: !,
    pub state: !,
}
