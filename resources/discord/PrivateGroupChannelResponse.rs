#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PrivateGroupChannelResponse {
    pub lastpintimestamp: !,
    pub id: !,
    pub type: !,
    pub flags: !,
    pub name: !,
    pub icon: !,
    pub managed: !,
}
