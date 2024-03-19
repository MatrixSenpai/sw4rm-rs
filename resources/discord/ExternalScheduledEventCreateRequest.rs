#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ExternalScheduledEventCreateRequest {
    pub privacylevel: !,
    pub scheduledendtime: !,
    pub entitytype: !,
    pub image: !,
    pub name: !,
    pub description: !,
    pub scheduledstarttime: !,
}
