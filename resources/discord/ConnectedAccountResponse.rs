#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConnectedAccountResponse {
    pub showactivity: !,
    pub twowaylink: !,
    pub verified: !,
    pub id: !,
    pub visibility: !,
    pub revoked: !,
    pub name: !,
    pub friendsync: !,
    pub type: !,
}
