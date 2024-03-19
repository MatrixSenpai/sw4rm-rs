#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MatchTimelineInfo {
    pub frame_interval: !,
    pub game_id: !,
}
