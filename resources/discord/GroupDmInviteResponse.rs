#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupDmInviteResponse {
    pub expiresat: !,
    pub approximatemembercount: !,
    pub code: !,
    pub maxage: !,
    pub createdat: !,
}
