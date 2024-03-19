#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteGuildResponse {
    pub description: !,
    pub nsfw: !,
    pub id: !,
    pub vanityurlcode: !,
    pub name: !,
    pub icon: !,
    pub premiumsubscriptioncount: !,
    pub banner: !,
    pub splash: !,
}
