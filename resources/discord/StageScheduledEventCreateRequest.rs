#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StageScheduledEventCreateRequest {
    pub scheduledendtime: !,
    pub entitytype: !,
    pub scheduledstarttime: !,
    pub privacylevel: !,
    pub name: !,
    pub description: !,
    pub image: !,
}
