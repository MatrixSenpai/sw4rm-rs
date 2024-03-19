#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VoiceScheduledEventCreateRequest {
    pub privacylevel: !,
    pub entitytype: !,
    pub name: !,
    pub image: !,
    pub scheduledendtime: !,
    pub scheduledstarttime: !,
    pub description: !,
}
