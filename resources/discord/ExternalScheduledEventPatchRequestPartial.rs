#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ExternalScheduledEventPatchRequestPartial {
    pub scheduledstarttime: !,
    pub scheduledendtime: !,
    pub image: !,
    pub name: !,
    pub privacylevel: !,
    pub description: !,
}
