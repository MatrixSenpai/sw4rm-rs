#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildRoleResponse {
    pub position: !,
    pub id: !,
    pub name: !,
    pub description: !,
    pub color: !,
    pub hoist: !,
    pub permissions: !,
    pub managed: !,
    pub mentionable: !,
    pub unicodeemoji: !,
    pub icon: !,
}
