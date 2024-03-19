#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildTemplateRoleResponse {
    pub hoist: !,
    pub mentionable: !,
    pub id: !,
    pub name: !,
    pub permissions: !,
    pub unicodeemoji: !,
    pub icon: !,
    pub color: !,
}
