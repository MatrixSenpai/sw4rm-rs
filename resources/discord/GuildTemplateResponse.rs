#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildTemplateResponse {
    pub isdirty: !,
    pub createdat: !,
    pub usagecount: !,
    pub updatedat: !,
    pub code: !,
    pub name: !,
    pub sourceguildid: !,
    pub description: !,
    pub creatorid: !,
}
