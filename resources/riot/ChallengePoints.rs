#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChallengePoints {
    pub current: !,
    pub max: !,
    pub percentile: !,
    pub level: !,
}
