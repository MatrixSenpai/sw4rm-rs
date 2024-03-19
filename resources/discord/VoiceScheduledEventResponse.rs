#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VoiceScheduledEventResponse {
    pub usercount: !,
    pub name: !,
    pub status: !,
    pub entitytype: !,
    pub id: !,
    pub image: !,
    pub privacylevel: !,
    pub scheduledendtime: !,
    pub scheduledstarttime: !,
    pub guildid: !,
    pub description: !,
}
