#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StageScheduledEventResponse {
    pub guildid: !,
    pub privacylevel: !,
    pub description: !,
    pub image: !,
    pub status: !,
    pub scheduledendtime: !,
    pub id: !,
    pub scheduledstarttime: !,
    pub entitytype: !,
    pub name: !,
    pub usercount: !,
}
