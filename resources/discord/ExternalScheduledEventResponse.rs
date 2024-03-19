#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ExternalScheduledEventResponse {
    pub guildid: !,
    pub usercount: !,
    pub scheduledstarttime: !,
    pub name: !,
    pub image: !,
    pub id: !,
    pub description: !,
    pub privacylevel: !,
    pub scheduledendtime: !,
    pub status: !,
    pub entitytype: !,
}
