#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NewMemberActionResponse {
    pub actiontype: !,
    pub description: !,
    pub channelid: !,
    pub title: !,
    pub icon: !,
}
