#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PrivateChannelResponse {
    pub lastpintimestamp: !,
    pub id: !,
    pub type: !,
    pub flags: !,
}
