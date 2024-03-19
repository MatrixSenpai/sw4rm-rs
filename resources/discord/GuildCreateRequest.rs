#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildCreateRequest {
    pub description: !,
    pub icon: !,
    pub name: !,
    pub systemchannelflags: !,
    pub region: !,
}
