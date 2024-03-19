#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ThreadMemberResponse {
    pub userid: !,
    pub flags: !,
    pub jointimestamp: !,
    pub id: !,
}
