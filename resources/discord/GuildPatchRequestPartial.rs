#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildPatchRequestPartial {
    pub icon: !,
    pub premiumprogressbarenabled: !,
    pub discoverysplash: !,
    pub region: !,
    pub ownerid: !,
    pub systemchannelflags: !,
    pub banner: !,
    pub name: !,
    pub description: !,
    pub homeheader: !,
    pub splash: !,
}
