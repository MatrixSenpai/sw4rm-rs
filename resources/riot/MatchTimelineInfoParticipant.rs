#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MatchTimelineInfoParticipant {
    pub participant_id: !,
    pub puuid: !,
}
