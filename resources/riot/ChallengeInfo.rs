#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChallengeInfo {
    pub challenge_id: !,
    pub players_in_level: !,
    pub achieved_time: !,
    pub percentile: !,
    pub position: !,
    pub level: !,
    pub value: !,
}
