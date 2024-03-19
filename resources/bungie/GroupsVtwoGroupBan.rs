#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupsVtwoGroupBan {
    pub date_expires: !,
    pub date_banned: !,
    pub comment: !,
    pub group_id: !,
}
