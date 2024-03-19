#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyRequestsActionsDestinyItemActionRequest {
    pub character_id: !,
    pub item_id: !,
    pub membership_type: !,
}
