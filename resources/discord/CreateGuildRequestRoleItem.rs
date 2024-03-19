#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateGuildRequestRoleItem {
    pub color: !,
    pub hoist: !,
    pub id: !,
    pub permissions: !,
    pub mentionable: !,
    pub unicodeemoji: !,
    pub name: !,
}
