#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ScheduledEventResponse {
    pub scheduledstarttime: !,
    pub image: !,
    pub description: !,
    pub status: !,
    pub guildid: !,
    pub privacylevel: !,
    pub scheduledendtime: !,
    pub id: !,
    pub entitytype: !,
    pub usercount: !,
    pub name: !,
}
