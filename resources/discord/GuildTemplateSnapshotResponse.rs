#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildTemplateSnapshotResponse {
    pub defaultmessagenotifications: !,
    pub preferredlocale: !,
    pub description: !,
    pub systemchannelflags: !,
    pub explicitcontentfilter: !,
    pub verificationlevel: !,
    pub afktimeout: !,
    pub name: !,
    pub region: !,
}
