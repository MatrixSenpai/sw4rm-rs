#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StageScheduledEventPatchRequestPartial {
    pub name: !,
    pub scheduledendtime: !,
    pub scheduledstarttime: !,
    pub description: !,
    pub image: !,
    pub privacylevel: !,
}
