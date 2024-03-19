#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteChannelResponse {
    pub id: !,
    pub name: !,
    pub icon: !,
    pub type: !,
}
