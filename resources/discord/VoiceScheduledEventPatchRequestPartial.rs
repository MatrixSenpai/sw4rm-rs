#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VoiceScheduledEventPatchRequestPartial {
    pub scheduledendtime: !,
    pub scheduledstarttime: !,
    pub image: !,
    pub name: !,
    pub description: !,
    pub privacylevel: !,
}
