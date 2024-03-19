#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteStageInstanceResponse {
    pub speakercount: !,
    pub topic: !,
    pub participantcount: !,
}
