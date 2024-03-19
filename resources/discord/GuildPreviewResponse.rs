#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildPreviewResponse {
    pub id: !,
    pub approximatepresencecount: !,
    pub icon: !,
    pub splash: !,
    pub description: !,
    pub homeheader: !,
    pub name: !,
    pub approximatemembercount: !,
    pub discoverysplash: !,
}
